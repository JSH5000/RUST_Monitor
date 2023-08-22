# Rusty Monitor
The goal of this project is simple:
<ol>
  <li>Provide an easy CLI to monitor edge devices.</li>
  <li>Provide a lightweight client to view incoming data from edge devices.</li>
  <li>Allow for agents to be deployed to monitor system resources.</li>
  <li>Allow for agents to send up resource monitoring to easy to deploy HTTPS server.</li>
</ol>

## Currently Working
- Sys monitoring. VERY basic (Only 1 graph for mem usage and basic sys info)
- Graphing mem live updates in console.

## Currently In Prog
- Adding full tabbed menu in for the following:
  <ul>
    <li>Sys Info (Page Mem, Swap, RAM, DISK, Usage)</li>
    <li>Process List (All process & PID's)</li>
    <li>Remote Viewing of Listed Items (ability to select from menu and see above tabs)</li>
  </ul>

## On Backlog TODO
- Add in auth system for agents | server | client (hard-ish)
- Add in the ability to remote run .sh or .ps1 scripts (via agent) (kinda ez ~ privledges and whatnot)
- Add in the ability to terminate processes by PID (via agent) (kinda ez)
- Add in the ability to alter UI color (ez)

## To run
- Verify rust is installed on your machine.
- For now, since the app is only the `host cli`, just git clone or download this repo.
- Then, run the command `cargo build` then `cargo run`. (I know the `cargo run` command will do both, I am just paranoid.
- The shell will then turn into the below image:
  ![Rust_Mon](https://github.com/JSH5000/RUST_Monitor/assets/39937127/8d765866-ce86-4180-b1a5-e1a8be19e39f)

## Credits
Using the awesome rust CLI UI library <a href="https://github.com/ratatui-org/ratatui">RATATUI</a>. Check em' out!
<br/>
Also using <a href="https://github.com/GuillaumeGomez/sysinfo">SYSINFO</a> for the system information. Give em' a look for your infomational needs!
