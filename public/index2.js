window.onload = function () {
  google.accounts.id.initialize({
    client_id:
      "428976116655-hmqs2oepkrklef5gsg21mbibacu4ppuj.apps.googleusercontent.com",
    callback: onSignIn,
  });
  const parent = document.getElementById("google_btn");
  google.accounts.id.renderButton(parent, { theme: "filled_blue" });
  google.accounts.id.prompt();
};

function onSignIn(e) {
  console.log(e);
  //   var id_token = googleUser.getAuthResponse().id_token;
  //   var profile = googleUser.getBasicProfile();
  //   console.log(id_token);
  //   console.log(profile);
  //   console.log("ID: " + profile.getId()); // Do not send to your backend! Use an ID token instead.
  //   console.log("Name: " + profile.getName());
  //   console.log("Image URL: " + profile.getImageUrl());
  //   console.log("Email: " + profile.getEmail()); // This is null if the 'email' scope is not present.
}
