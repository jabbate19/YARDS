use yew::prelude::*;
use yew_oauth2::prelude::*;
use yew_oauth2::openid::{use_auth_agent, Additional, Config}; // use `openid::*` when using OpenID connect
use crate::context::openid::OAuth2;
#[function_component(MyApplication)]
fn my_app() -> Html {
  let config = Config {
    client_id: "yards-frontend-public".into(),
    issuer_url: "https://sso.csh.rit.edu/auth/realms/csh".into(),
    additional: Additional {
        end_session_url: None,
        after_logout_url: None,
        post_logout_redirect_name: None
    }
    //auth_url: "https://sso.csh.rit.edu/auth/realms/csh/protocol/openid-connect/auth".into(),
    //token_url: "https://sso.csh.rit.edu/auth/realms/csh/protocol/openid-connect/token".into(),
  };

  html!(
    <OAuth2 config={config} scopes={vec!["openid".to_string()]}>
      <MyApplicationMain/>
    </OAuth2>
  )
}

#[function_component(MyApplicationMain)]
fn my_app_main() -> Html {
  let agent = use_auth_agent().expect("Must be nested inside an OAuth2 component");

  let login = {
    let agent = agent.clone();
    Callback::from(move |_| {
      let _ = agent.start_login();
    })
  };
  let logout = Callback::from(move |_| {
    let _ = agent.logout();
  });

  html!(
    <>
      <Failure><FailureMessage/></Failure>
      <Authenticated>
        <img src="https://plug.csh.rit.edu/data"/>
        <h1>{ "Hello, world!" }</h1>
        <button onclick={logout}>{ "Logout" }</button>
      </Authenticated>
      <NotAuthenticated>
        <button onclick={login}>{ "Login" }</button>
      </NotAuthenticated>
    </>
  )
}

fn main() {
    yew::Renderer::<MyApplication>::new().render();
}
