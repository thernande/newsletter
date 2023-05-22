var id_token;
var profile;

function init() {
  gapi.load("auth2", function () {
    client_id = {
      client_id:
        "428976116655-9n8oggl3j3q0m8qkrndmc84e1lvg7nh7.apps.googleusercontent.com",
    };
    gapi.auth2.init(client_id).then(function () {
      renderButton();
    });
  });
}

function onSignIn(googleUser) {
  console.log("entra");
  var id_token = googleUser.getAuthResponse().id_token;
  axios.post("./api/v1/users/login", { token: id_token });
  var profile = googleUser.getBasicProfile();
  console.log(id_token);
  console.log(profile);
  console.log("ID: " + profile.getId()); // Do not send to your backend! Use an ID token instead.
  console.log("Name: " + profile.getName());
  console.log("Image URL: " + profile.getImageUrl());
  console.log("Email: " + profile.getEmail()); // This is null if the 'email' scope is not present.
}

function signOut() {
  var auth2 = gapi.auth2.getAuthInstance();
  console.log(auth2);
  auth2.signOut().then(function () {
    console.log("User signed out.");
  });
}
