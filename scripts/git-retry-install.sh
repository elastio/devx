# This script inslalls git wrapper with the retry logic for the commands which may work with network.
# The real git is always in the '/usr/bin/'. Our retry wrapper will be in the '/usr/local/bin/'.
# The second path is always earlier in the $PATH, so our wrapper will be used first.

#!/bin/bash

mkdir -p /usr/local/bin
cat <<'AAA' > /usr/local/bin/git
#!/bin/bash
GIT=/usr/bin/git
# Max delay of 45 sec, 15 retries per 3 sec
RETRIES=15
DELAY=3
for (( i=0; i<=$RETRIES; i++ )) ; do
    $GIT "$@"
    ret=$?
    # don't retry if git command is not about network, like 'git log' or 'git diff' or
    # getting branch name like in my .bashrc 'rev-parse --abbrev-ref HEAD'
    cmd=$(echo "$@" | awk '{ print $1 }')
    case $cmd in
        clone | commit | merge | rebase | tag | fetch | pull | push) ;;
        *) break ;;
    esac
    # git returns 1 in case if command is incorrect, for other cases
    # it returns 128 or 130. So, we should not retry on 1.
    [ $ret -eq 0 ] || [ $ret -eq 1 ] || [ $i = $RETRIES ] && break
    >&2 echo "ERROR: command 'git $@' failed with the code $ret. Retrying $(($RETRIES-$i)) time(s) in $DELAY sec..."
    sleep $DELAY
done
exit $ret
AAA

chmod +x /usr/local/bin/git
