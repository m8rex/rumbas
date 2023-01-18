## Step 3: Try compiling the first question

Now we can compile the question. To do this, we need to open a terminal in the folder of your rumbas project. Then we can run the following command:

```bash
rumbas check questions/first_question.yaml
```

This command will now show an error. This is because our rumbas repository is missing a crucial file. This file is the `rumbasrc.yaml` file. This file contains the settings for the rumbas repository. Create the file and add the following content:

```yaml
{{#include ../../../../examples/first_question/.rumbasrc.yaml}}
```

This file is used to check whether you are using the right rumbas version for the rumbas project. If you are using the wrong version, you will get an error. 