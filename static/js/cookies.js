let cookies_array = document.cookie.split("; ")
let cookies = new Map()
for (let cookie of cookies_array) {
    let pair = cookie.split("=")
    cookies.set(pair[0], pair[1])
}
