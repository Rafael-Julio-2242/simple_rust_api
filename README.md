# SIMPLE RUST API


This is a simple rust api, the ideia is that we have users and each user as notes

## We have the following routes for the User

- Get all users (get_all_users) :  `get("/users")`
    - Expects nothing, returns a Vector with all the users in the database
- Get a especific user (get_user) : `get("/users/{id}")`
  - Expects the id of the user as a URL Paramenter, returns the especified user
- Create a new User : `post("/users/new_user")`
  - Expects the name and the password of the new user, returns if it was succesfull or not
      - body: `{
                "id": 0,
                "name": "",
                "password": ""
              }`
- Delete a User : `delete("/users/{id}")`
  - Expects the id of the user as a URL Parameter, return if it was succesfull or not
- Update a User : `path("/users/update_user")`
  - Expects, in the least, the "id" of the user and at least one of both: "name" or "password" in the body, return it it was successfull or not
    - body: `{
              "id": 0,
              "name": "",
              "password": ""
            }`


## We have the following routes for the Note

- Get all the notes (get_all_notes) : `get("/notes")`
  - Expects nothing, returns a Vector with all the notes in the database
- Get all the notes of a especific user (get_user_notes) : `get("/notes/{user_id})`
  - Expects the user id in the URL Parameter, returns a Vector with the notes of the user
- Create a new Note (new_note) : `post("/notes/new_note")`
  - Expects the "title", the "description" and the "user_id" in the body of the req, returns if it was successfull or not
    - body: `{
              "title": "",
              "description": "",
              "user_id": 0
            }`
- Delete a Note (delete_note) : `delete("/notes/{id}")`
  - Expects the id of the note to delete, returns if it was succesfull or not
- Update a Note (update_note) : `patch("/notes/update_note")`
  - Expects the id of the note to be changed and in the least the "title" and / or the "description" of the note, returns if it was successfull or not
    - body: `{
              "id": 0,
              "title": "",
              "description": "",
            }`
 


It is using sqlite as a simple database, just for purposes of making the this rep fast

Have fun :D

