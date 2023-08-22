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
- Add in auth system for agents | server | client
- Add in the ability to remote run .sh or .ps1 scripts (via agent)
- Add in the ability to terminate processes by PID (via agent)
