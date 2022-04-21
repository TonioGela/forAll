## forAll
Have you ever wanted to run a command in all the current working directory child directories? In parallel using multithreading? Here is your way to do it.

### Usage 
`forAll <command>` outputs something like

```
<#1 child folder name in green if exitcode 0 otherwise red>:
<stdout of command in #1 child>
<stderr of command in #1 child>

...

<#N child folder name in green if exitcode 0 otherwise red>:
<stdout of command in #N child>
<stderr of command in #N child>
```

the exitcode is the || of all the exitcodes