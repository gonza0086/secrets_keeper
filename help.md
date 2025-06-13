Zerbero
Terminal based secures password manager

usage:
- add
Add random password for a given app
    
    zerbero -a app_name [-o]

- update
Update the password of the given app

    zerbero -u app_name [-o]

[-o] Indicates whether the password parameters should be configued manually or not

- get
Get the password of a given app

    zerbero -g app_name

- delete
Delete passwords of a given app

    zerbero -d app_name

- list
List all passwords and apps

    zerbero -l
