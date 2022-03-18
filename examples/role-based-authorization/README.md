# Role Base Authorization Example

In this example for role based permission check, basic authentication is used with 3 users.
Each user has a different role - *Administrator, Moderator and User*.

There are 3 pages served:
- Only for Administrators `admin:1` <http://localhost:8888/admin>
- For Moderators and higher `moderator:2` <http://localhost:8888/mod>
- For Logged in users `user:3` <http://localhost:8888/>

# Running the App
```cargo run``` and go to <http://localhost:8888/>
