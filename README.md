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
zerbero -k <FILE_KEY> --<VERB> <APP_NAME>

To list all passwords the <APP_NAME> is not needed.

### Verbs
Add: --add | -a

Get: --get | -g

Update: --update | -u

Delete: --delete | -d

List: --list | -l

## Todo
- Remove key from the command and add it as an input
- Make password characteristics a configuration provided when a passwword is being created
- save for a time period the master key to avoid writing it so many times just like sudo works
