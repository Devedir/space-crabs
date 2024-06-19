function logout() {
    document.cookie = "user_id=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;"
    document.cookie = "login=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;"
    document.cookie = "roles=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;"
    location.href = "/";
}
