var searchIndex = JSON.parse('{\
"devx_cmd":{"doc":"<code>devx-cmd</code> provides more convenient primitives for spawning …","t":[3,3,3,4,6,13,13,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,14,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,14,11,11,14,11,11,11,11,14,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11],"n":["Child","Cmd","Error","Ostream","Result","StdErr","StdOut","arg","arg2","args","bin","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone_into","clone_into","cmd","cmd","current_dir","drop","env","eq","fmt","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","get_args","get_bin","get_current_dir","hash","into","into","into","into","log_cmd","log_err","lookup_in_path","new","read","read","read","read_bytes","read_bytes","read_bytes","read_bytes_no_wait","read_no_wait","replace_arg","run","run","spawn","spawn_piped","spawn_with","stdin","stdin_bytes","stdout_lines","to_owned","to_owned","to_string","to_string","to_string","to_string","try_at","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","wait"],"q":["devx_cmd","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["Wraps <code>std::process::Child</code>, kills and waits for the …","More convenient version of <code>std::process::Command</code>. Allows …","Opaque error which happened during command execution.","Defines the kind of standard process output stream.","Shortcut for <code>Result<T, devx_cmd::Error></code>","<code>stderr</code> is the stream for human-readable messages not …","<code>stdout</code> is the default output where process outputs its …","Appends a single argument to the list of arguments passed …","Same as <code>cmd.arg(arg1).arg(arg2)</code>. This is just a …","Extends the array of arguments passed to the child …","Set binary path, overwrites the path that was set before.","","","","","","","","","","","","","Returns the <code>Cmd</code> that was originally used to create this …","Create a <code>Cmd</code> with the given binary and arguments.","Set the current directory for the child process.","","Inserts or updates an environment variable mapping.","","","","","","","","","","","","","Returns the currently configured list of command line …","Retuns the currently configured binary path.","Returns the currently configured current process …","","","","","","When set to some <code>log::Level</code> the command with its …","When set to some <code>log::Level</code> the invocation error will be …","Returns a command builder for the given <code>bin_name</code> only if …","Returns a command builder that invokes the binary at <code>bin</code>. …","Waits for the process to finish and returns all that it …","Same as <code>cmd.spawn_piped()?.read()</code> See <code>Child::read()</code> for …","Shortcut for <code>cmd!(...).read()</code>.","Same as <code>Child::read()</code> but reads any bytes sequence from …","Same as <code>cmd.spawn_piped()?.read_bytes()</code> See …","Shortcut for <code>cmd!(...).read_bytes()</code>.","Same as <code>Child::read_no_wait()</code>, but reads raw bytes.","Same as <code>Child::read()</code>, but doesn’t wait for the process …","Replaces the argument at the given index with a new value.","Same as <code>cmd.spawn()?.wait()</code> See <code>Child::wait()</code> for details.","Shortcut for <code>cmd!(...).run()</code>.","Spawns a child process returning a handle to it. The …","Spawns a child process returning a handle to it. Child’…","More flexible version of <code>spawn</code> methods that allows you to …","Sets the string input passed to child process’s <code>stdin</code>. …","Sets the bytes input passed to child process’s <code>stdin</code>. …","Returns an iterator over the lines of data output to …","","","","","","","Returns a command builder if there is some file available …","","","","","","","","","","","","","Waits for the process to finish. Returns an error if the …"],"i":[0,0,0,0,0,1,1,2,2,2,2,3,4,2,1,3,4,2,1,2,1,2,1,3,0,2,3,2,1,3,4,4,2,2,1,1,3,4,2,1,2,2,2,1,3,4,2,1,2,2,2,2,3,2,0,3,2,0,3,3,2,2,0,2,2,2,2,2,3,2,1,3,4,2,1,2,3,4,2,1,3,4,2,1,3,4,2,1,3],"f":[null,null,null,null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["cmd",3]],[[],["ostream",4]],[[]],[[]],[[],["cmd",3]],null,[[]],[[]],[[]],[[["ostream",4]],["bool",15]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[]],[[]],[[]],[[],["path",3]],[[],[["option",4,["path"]],["path",3]]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[["str",15]],["option",4]],[[]],[[],[["result",6,["string"]],["string",3]]],[[],[["result",6,["string"]],["string",3]]],null,[[],[["vec",3,["u8"]],["result",6,["vec"]]]],[[],[["vec",3,["u8"]],["result",6,["vec"]]]],null,[[["ostream",4]],[["vec",3,["u8"]],["result",6,["vec"]]]],[[["ostream",4]],[["result",6,["string"]],["string",3]]],[[["usize",15]]],[[],["result",6]],null,[[],[["result",6,["child"]],["child",3]]],[[],[["result",6,["child"]],["child",3]]],[[["stdio",3]],[["result",6,["child"]],["child",3]]],[[]],[[["vec",3,["u8"]],["u8",15]]],[[]],[[]],[[]],[[],["string",3]],[[],["string",3]],[[],["string",3]],[[],["string",3]],[[],["option",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["result",6]]],"p":[[4,"Ostream"],[3,"Cmd"],[3,"Child"],[3,"Error"]]},\
"devx_pre_commit":{"doc":"<code>devx-pre-commit</code> provides utilities for creating git …","t":[3,11,11,11,11,5,11,5,11,11,11,11,11,11,11,11,11],"n":["PreCommitContext","borrow","borrow_mut","from","from_git_diff","install_self_as_hook","into","locate_project_root","retain_staged_files","rustfmt","stage_new_changes","staged_files","staged_rust_files","touched_crates","try_from","try_into","type_id"],"q":["devx_pre_commit","","","","","","","","","","","","","","","",""],"d":["Represents the API entrypoint of the git pre-commit hook. …","","","","Creates the git pre-commit context acquiring the staged …","Copies the <code>std::env::current_exe()</code> file to …","","Searches for a project root dir, which is a directory …","Accepts a function predicate that accepts a relative path …","Runs <code>cargo fmt</code> against the <code>Self::touched_crates()</code>","Pushes the changes introduced to staged files in the …","Returns an iterator over all the files staged for the …","Returns an iterator over all staged files with <code>.rs</code> …","Returns the names of the crates that contain …","","",""],"i":[0,1,1,1,1,0,1,0,1,1,1,1,1,1,1,1,1],"f":[null,[[]],[[]],[[]],[[],["result",6]],[[],["result",6]],[[]],[[],[["pathbuf",3],["result",6,["pathbuf"]]]],[[]],[[],["result",6]],[[],["result",6]],[[]],[[]],[[],[["string",3],["hashset",3,["string"]]]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]]],"p":[[3,"PreCommitContext"]]},\
"xtask":{"doc":"","t":[0,5,5],"n":["pre_commit","install_hook","run_hook"],"q":["xtask","xtask::pre_commit",""],"d":["","",""],"i":[0,0,0],"f":[null,[[],["result",6]],[[],["result",6]]],"p":[]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};