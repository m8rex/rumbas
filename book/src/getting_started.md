# Getting Started

To follow along with this Getting Started, you will need an installation of rumbas.

```admonish question title="Task"
Follow the steps in [the installation guide](./installation.html#running-the-last-released-version) to start a github codespace with the latest released version of rumbas installed.
```

## Creating a new rumbas project

```admonish info
You can run the command `rumbas init` to create a new rumbas project.
```

```admonish info
In the VS Code setup (local or Github Codespaces) you can run this command by running the init task. You can do this in two ways:
- Type <kbd>CTRL</kbd> + <kbd>SHIFT</kbd> + <kbd>P</kbd>, then type `Tasks: Run Task` and select `init`.
- Click on the `create rumbas project` button on the taskbar at the bottom of the screen.
![The create rumbas project button](images/create_rumbas_project_button.png)
```


```admonish question title="Task"
Run the command to create a new rumbas project.
```

```admonish info
{{#include ./reference/folder_structure_folders.md}}
```

## Creating our first question

We will create a question about the price for shoes after a discount.

To add a question, we will add a file to the `questions` folder. The file should have the extension `.yaml`. Let's name the file `shoes.yaml`

```admonish info
Adding a file the `questions` folder can be done in many ways. The easiest way is to right click on the `questions` folder and select `New File`. You can then name the file `shoes.yaml`.
```

```admonish question title="Task"
Create the file `questions/shoes.yaml` (this is a `shoes.yaml` file in the `questions` folder).
```

We can specify the shoe discount question with the following yaml code:

```yaml
{{#include ../../examples/percentages-with-templates/questions/percentages-shoes-without_templates_en.yaml}}
```

```admonish question title="Task"
Add the yaml code to the `questions/shoes.yaml` file.
```

## Compiling our first question

```admonish info
You can run the command `rumbas compile questions/shoes.yaml` to compile the questions.
```

```admonish info
In the VS Code setup (local or Github Codespaces) you can run this command by running the compile task. You can do this in two ways:
- Type <kbd>CTRL</kbd> + <kbd>SHIFT</kbd> + <kbd>P</kbd>, then type `Tasks: Run Task` and select `compile`.
- Click on the `Compile file` button on the taskbar at the bottom of the screen. Make sure that you have opened the `questions/shoes.yaml` file.
```

```admonish question title="Task"
Compile the question.
```

## Viewing the compiled question

```admonish info
The compilated step generated a folder containing the html and js files for the question in the `_output` folder in your project.

The exact path will be `_output/en/questions/shoes`.
```

```admonish info
To view this file, you need to host the `_output` folder. 

The easiest way to do this in VSCode is by starting the Live Server extension. You can do this by clicking on the `Go Live` button on the taskbar at the bottom of the screen. It is located on the right side.

This will open a browser window with the contents of the `_output` folder.

Click through the folder structure to reach you question.
```

```admonish question title="Task"
Start the live server and try the question in the browser window that opens.
```

```admonish question title="Task"
Answer the question and check if the answer is correct.

Use the `Try another question like this one` button to try another question, with different random values.
```