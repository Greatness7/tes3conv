# tes3conv

Get the latest version from the [releases](https://github.com/Greatness7/tes3conv/releases) page.

A simple command-line tool that lets you convert TES3 plugin files (`.esp`) into JSON files (`.json`), and vice-versa.

```
Convert TES3 plugins (.esp) into JSON files (.json), and vice-versa.

Usage: tes3conv "test.esp" "test.json"

Arguments:
  <INPUT>   Sets the input file. Pass - to use stdin.
  [OUTPUT]  Sets the output file. Omit to use stdout.

Options:
  -c, --compact    Compact json output (skip indentation).
  -o, --overwrite  Overwrite output without making backups.
  -h, --help       Print help
```

---

The primary motivation for this tool is to enabled TES3 projects to better make use of version control systems like `Git`. To enable automatic git integration you will have to do some additional setup, as detailed below.

Plugins are also provided for automatic integration with the [WinMerge](https://winmerge.org/?lang=en) and [Beyond Compare](https://www.scootersoftware.com/) applications.

---

**Git Integration Instructions: (Windows)**

- **Step 1:** Navigate to your "home" directory.  

  You can enter `%HOMEPATH%` in the explorer address bar as a shortcut.  
  *( Example: `C:/Users/Todd/` )*

- **Step 2:** Set up `tes3conv.exe` as a conversion tool.  

  In your home directory find (or create) a `.gitconfig` file and open it in a text editor.  
  Add the following lines at the end:
  ```
  [diff "tes3"]
      textconv = "C:/Path/To/Your/tes3conv.exe"
  ```
  *( Note: Make sure the filepath is correct for your system )*  
  *( Note: You must use forward slashes, not backslashes! )*


- **Step 3:** Set the conversion tools associated file types.  

  In your home directory find (or create) a `/.config/git/attributes` file.
  
  If the `/.config/git/` directories did not exist, create them, and then create the `attributes` file within.
  
  Open the `attributes` file in a text editor and add the following line:
  ```
  *.[eE][sS][mpMP] diff=tes3
  ```
  *( Note: The file name must be just `attributes`. No dots/file extension! )*


- **Step 4:** Check if it works!

  Your git tools should now automatically show proper diffs for `.esp` and `.esm` files.

  ![](assets/example.png?raw=true "Title")

    *( Note: Not all git clients support this feature, if yours isn't working try [GitHub Desktop](https://desktop.github.com) )*.

---

**WinMerge Integration Instructions: (Windows)**

- **Step 1:** Add `tes3conv.exe` to your PATH environment variable.

  You can find instructions for how to do this [here](https://www.howtogeek.com/118594/how-to-edit-your-system-path-for-easy-command-line-access/).

- **Step 2:** Install the TES3 WinMerge plugin.

  Copy the [CompareTES3Files.sct](https://github.com/Greatness7/tes3conv/tree/master/plugins/WinMerge) file from this repository into your WinMerge installation's `/MergePlugins/` directory.

- **Step 3:** Enable the TES3 WinMerge plugin.

  Start up `WinMerge` and open the `Plugins->Plugin Settings` menu. Ensure the `Enable Plugins` box is checked, and then check box the next to the `CompareTES3Files.sct` entry.

  After that you may also want to enable the `Plugins->Automatic Unpacking` setting.

---

**Beyond Compare Integration Instructions: (Windows)**

- **Step 1:** Add `tes3conv.exe` to your PATH environment variable.

  You can find instructions for how to do this [here](https://www.howtogeek.com/118594/how-to-edit-your-system-path-for-easy-command-line-access/).

- **Step 2:** Install the TES3 Beyond Compare plugin.

  Start up `Beyond Compare` and open its `Tools->Import Settings` menu. When it asks you to select an import file choose the [TES3Format.bcpkg](https://github.com/Greatness7/tes3conv/tree/master/plugins/Beyond%20Compare) from this repository. After importing make sure you check the box for the `TES3` file format.

- **Step 3:** Enable the TES3 Beyond Compare plugin.

  In the `Beyond Compare` interface open up `Tools->File Formats` and check the box next to the `TES3` entry.
