use leptos::{
    component, create_effect, create_resource, create_signal, view, IntoView, SignalGet, SignalSet,
    Suspense,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Claims {
    user_id: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct Profile {
    name: String,
}

struct AuthService;

impl AuthService {
    fn get_current_user() -> Claims {
        Claims::default()
    }
}

type ResultProfile = Result<Profile, ()>;

#[allow(unused)]
async fn get_user_profile(user_id: String) -> ResultProfile {
    let profile = Profile {
        name: String::from("John Snow"),
    };

    Ok(profile)
}

async fn fetch_profile(user: Claims) -> ResultProfile {
    let result = get_user_profile(user.user_id).await;
    // I can see the log in the browser console!
    leptos::logging::log!("Profile: {:?}", result);
    result
}

#[component]
pub fn SuspensePage() -> impl IntoView {
    let (aut_user, set_auth_user) = create_signal(Claims::default());

    create_effect(move |_| {
        // Get the user from the local storage once the code runs in browser
        let user = AuthService::get_current_user();
        // Set the user and trigger re-fetching the resource
        set_auth_user.set(user);
    });

    let user = create_resource(move || aut_user.get(), fetch_profile);

    view! {
        <div class="columns">
            <div class="column">
                <h1 class="title is-5">"My Profile"</h1>
                <h2 class="subtitle is-6">"Your personal data"</h2>
                <div class="content">
                    <Suspense fallback=move|| {
                        view! { <p>"Loading data..."</p> }
                    }>
                        {move || {
                            user.get()
                                .map(|user| {
                                    view! { <p>{user.unwrap().name}</p> }
                                })
                        }}

                    </Suspense>
                </div>
            </div>
        </div>
    }
}
