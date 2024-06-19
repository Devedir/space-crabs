function signup() {
    fetch(location.href, {
        method: "POST",
        body: cookies.get("user_id"),
        headers: {
            "Content-type": "text/plain; charset=UTF-8"
          }
    })
    location.reload()
}
