# Zerbero
Terminal based secure password manager. All passwords are stores in an encrypted file using rust's crate Cocoon. The encryption key must be specified the first time a password is added.

### Features
- Add password for a specified app
- Get password of a specified app
- Update password for a specified app
- Delete password for a specified app
- List all passwords

### Execution
In order to execute one of this features the following command should be used:
zerbero --<VERB> <APP_NAME> [-o]

To list all passwords the <APP_NAME> is not needed.

The -o option allows to specify parameters for the password creation. Only works with verbs add and update

### Verbs
Add: --add | -a

Get: --get | -g

Update: --update | -u

Delete: --delete | -d

List: --list | -l
