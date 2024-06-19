async function delete_user(user_id) {
    let response = await fetch("/user/" + user_id, {
        method: "DELETE",
        headers: {
            "Content-type": "text/plain; charset=UTF-8"
          }
    })
    location.reload()
}
