## Instruction
1. start or stop Postgres with 
```
pg_ctl -D /usr/local/var/postgres start
pg_ctl -D /usr/local/var/postgres stop
```

2. Run
```
cargo run
```

3. Enjoy!

## Endpoint

- ```GET /users``` - return all users
- ```POST /users``` - Create new user
- ```PUT /users/{id}``` - Update user's info on specified id
- ```DELETE /users``` - Delete user on specifiled id

### TODO
- [ ] Implement JWT