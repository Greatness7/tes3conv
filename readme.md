# tes3conv

Get the latest version from the [releases](https://github.com/Greatness7/tes3conv/releases) page.

A simple command-line tool that lets you convert TES3 plugin files (`.esp`) into JSON files (`.json`), and vice-versa.

```
tes3conv 0.0.6
Convert TES3 plugins (.esp) into JSON files (.json), and vice-versa.

USAGE:
    tes3conv "test.esp" "test.json"

FLAGS:
    -c, --compact      Compact json output (skip indentation).
    -o, --overwrite    Overwrite output without making backups.
    -h, --help         Prints help information
    -V, --version      Prints version information

ARGS:
    <INPUT>     Sets the input file. Pass - to use stdin.
    <OUTPUT>    Sets the output file. Omit to use stdout.
```

---

The primary motivation for this tool is to enabled TES3 projects to better make use of version control systems like `Git`. To enable automatic git integration you will have to do some additional setup, as detailed below.


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
