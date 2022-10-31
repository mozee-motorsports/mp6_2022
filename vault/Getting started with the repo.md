# Getting starting with the repo
## Cloning the repo to your local machine
Create an empty directory you have easy access to. You will need to navigate to it and open the git bash client from it often so create it in a spot you can easily navigate to. 
Next, open git bash inside your empty directory. Right clight and select "git bash here".
Next, simply type "git clone" and then the HTTPS link to the git repository. It will automatically download and store the repository to the local you currently have open in git bash. 
## Pulling updates from the repo
To pull updates from the repo, git bash inside wherever your local repository is stored, navigate inside the repository, and enter "git pull". You will know you are inside the right location if you see (*branch name*) next to the pathname. 
## Switching branches
If you are on a developement branch and wish to change to the main branch, simply type "git checkout main". Vise versa, if you are on main and wish to switch to a existing development branch type "git checkout development".
### Creating a branch
If you wish to create a branch, simply type out "git checkout -b *name of your new branch*". This will create your branch and automatically place you inside of it.
### Pushing your branch
If you wish to push your branch to the remote repository you use the following commands: git commit -a -m 'your message here'. Once you have commited, you will be able to push to origin. The command git push -u origin *name of the branch*. This will make the branch available on the remote repository to be pulled from and pushed to.
