## Step 5: Compiling our first exam

Now we can compile the exam. To do this, we need to open a terminal in the folder of your rumbas project. Then we can run the following command:

```bash
rumbas check exams/first_exam.yaml
```

```admonish question title="Task"
Run the command above in the terminal.
```

```admonish success
It should result in `All 1 checks passed!`. This means that the exam is valid and can be compiled.
```


```bash
rumbas compile exams/first_exam.yaml
```

```admonish question title="Task"
Run the command above in the terminal.
```

This command will create a folder in `_output/en/exams/first_exam` with the compiled exam. 

```admonish warning
To view this in the browser you should run a local webserver that hosts the `_output` folder.

Some options are:
- Live server extension in visual studio code (this is installed in our Github Codespaces setup). Click on the `Go Live` option in the right bottom corner.
- The [Web Server for Chrome](https://chrome.google.com/webstore/detail/web-server-for-chrome/ofhbbkphhbklhfoeikjpcbhemlocgigb) extension 
- Python: execute `python -m http.server` in the `_output` folder
- ...
```