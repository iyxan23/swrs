# Reading a Sketchware project's data
This documentation shows you how to read a sketchware project's data.

## Where are they
Sketchware stores its data on the folder `INTERNAL/.sketchware/`

There are 3 directories in which sketchware stores its projects in:

 - `mysc/{project-id}` -> Where the generated code of a project are cached, you need to run the project to let sketchware generate them.
 - `mysc/list/{project-id}` -> This folder contains the file named `project` which contains the project metadata (such as: app name, package name, etc).
 - `data/{project-id}` -> This folder contains the data of the project as shown in the table below

   | Filename | Description |
   | -------- | ----------- |
   | `view`   | Stores the layout and screens contained in the project |
   | `logic`  | Stores the blocks, events, components, variables contained in the project |
   | `library` | Stores the information about the libraries used in the project, like firebase, appcompat, etc |
   | `file` | Declares screens & customviews (where its data are contained in `view` & `logic`) + its options; such as orientation, keyboard settings, etc |
   | `resource` | Stores resources used in the project |

## Decrypting
If you did try reading those files directy, you'll get a mess of binary data. It's because those files are encrypted by Sketchware. Decrypting these files are very straightforward, use AES128 with the CBC method with PKCS#5 or PKCS#7 padding (if you're encrypting and you're using PKCS#7, make sure to have 8 as the block size) and set the key & IV to be "sketchwaresecure".
