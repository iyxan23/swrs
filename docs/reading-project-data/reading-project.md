# Reading `project`
This `project` file simply contains the project metadata (such as app name, project name, app version name, version code, etc) and is stored in `INTERNAL/.sketchware/mysc/list/{project-id}/project`.

It literally is just a json file, so just plug it up to your json parser I guess.

Sample `project` file:
```
{"custom_icon":true,"sc_ver_code":"2","my_ws_name":"Paint","color_accent":-1.6740915E7,"my_app_name":"SketchOn","sc_ver_name":"2.0","sc_id":"721","color_primary":-1.6740915E7,"color_control_highlight":5.36907213E8,"color_control_normal":-1.1026706E7,"sketchware_ver":150.0,"my_sc_reg_dt":"20190825022703","my_sc_pkg_name":"com.siglar.sketchon","color_primary_dark":-1.674323E7}
```

> If you're wondering why it doesn't look like JSON at all: [you need to decrypt it](../reading-a-sketchware-project.md#Decrypting)

**Fields:**

| Name | Description |
| ---- | ----------- |
| `str sc_id` | The project id used in the local device. If you're trying to re-import a sketchware project, you must set this id so that sketchware won't get confused |
| `str my_app_name` | The actual name that will be used in the genereated APK |
| `str my_ws_name` | The name used by sketchware to display on the top left corner when you're editing the project, I usually call it as a "workspace name". This is called as "project name" on the create project dialog, usually left out as "NewProject7" or something like that. NOTE: This is NOT the app name |
| `str my_sc_pkg_name` | This stores the package name that will be used in the generated APK |
| `str (int) sc_ver_code` | Version code of this project. Basically an incrementing integer that computer used to differentiate between different APK versions without having pain with the version name (a version string that is not standardized and is different for different APKs). TL;DR: when new apk released, version code increase |
| `str sc_ver_name` | The version name that will be used in the generated APK. Do note that this is different from version code, see its description for more details |
| `int my_sc_reg_dt` | This stores the date when this project was created. It's structured in a weird way, but it basically is `YYYYMMDDHHmmSS` (being that Y: year, M: month, D: day, H: hour, m: minute, S: second). |
| `float (int) sketchware_ver` | The sketchware version that is used to generate and edit this project. For a list of sketchware versions mapped by their app versions, refer to [sketchware-versions](../sketchware-versions.md) |
| `boolean custom_icon` | Used to indicate if this project uses a custom icon (which I haven't re-searched yet, so I might update this later) |
| `int (color) color_primary` | The color primary used by the project stored as an integer, for more information on how to parse color values like this, refer to [reading colors](../reading-colors.md) |
| `int (color) color_primary_dark` | The color primary dark used by the project stored as an integer, for more information on how to parse color values like this, refer to [reading colors](../reading-colors.md) |
| `int (color) color_accent` | The color accent used by the project stored as an integer, for more information on how to parse color values like this, refer to [reading colors](../reading-colors.md) |
| `int (color) color_control_highlight` | The color control highlight used by the project stored as an integer, for more information on how to parse the color values like this, refer to [reading colors](../reading-colors.md) |
| `int (color) color_control_normal` | The color control normal used by the project stored as an integer, for more information on how to parse the color values like this, refer to [reading colors](../reading-colors.md) |
