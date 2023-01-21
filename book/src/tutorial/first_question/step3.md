## Step 3: Try compiling the first question

Now we can compile the question. To do this, we need to open a terminal **in the folder of your rumbas project**. Then we can run the following command:

```bash
rumbas check questions/first_question.yaml
```

```admonish question title="Task"
Run the command above in the terminal.
```


This command will now show an error about the question not belonging to a rumbas project. This is because our rumbas repository is missing a crucial file. This file is the `.rumbasrc.yaml` - mind the dot (`.`) at the start - file. This file contains the settings for the rumbas repository.

```admonish question title="Task"
 Create the file and add the content below.
```

```yaml
{{#include ../../../../examples/first_question/.rumbasrc.yaml}}
```

```admonish info
The `.rumbasrc.yaml` file is used to check whether you are in a rumbas project and whether you are using the right rumbas version for the rumbas project. If you are using the wrong version, you will get an error. 
```