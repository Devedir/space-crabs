let cookies_array = document.cookie.split("; ")
let cookies = new Map()
for (let cookie of cookies_array) {
    let pair = cookie.split("=")
    cookies.set(pair[0], pair[1])
}

if (cookies.has("user_id")) {
    document.getElementById("account-anchor")
        .setAttribute("href", "/account/" + cookies.get("user_id"))
    document.getElementById("account-header").innerHTML = "Moje konto"
    document.getElementById("users-anchor").removeAttribute("hidden")
} else {
    document.getElementById("account-header").innerHTML = "Zaloguj się"
}
