use std::collections::HashMap;
use swrs::parser::logic::Logic;
use swrs::parser::logic::component::{Component};
use swrs::parser::logic::event::{Event, EventPool};
use swrs::parser::logic::list_variable::{ListVariable, ListVariablePool};
use swrs::parser::logic::more_block::{MoreBlock, MoreBlockPool};
use swrs::parser::logic::variable::{Variable, VariablePool, VariableType};
use swrs::parser::Parsable;

// Variables =======================================================================================

#[test]
fn parse_variable_0() {
    let input = "1:my_int";

    let result = match Variable::parse(input) {
        Ok(val) => val,
        Err(err) => panic!("Failed to parse variable: {}", err)
    };

    assert_eq!(
        Variable {
            name: "my_int".to_string(),
            r#type: VariableType::Integer
        },
        result
    );
}

#[test]
fn parse_variable_1() {
    let input = "3:variable_with_a_very_long_name";

    let result = match Variable::parse(input) {
        Ok(val) => val,
        Err(err) => panic!("Failed to parse variable: {}", err)
    };

    assert_eq!(
        Variable {
            name: "variable_with_a_very_long_name".to_string(),
            r#type: VariableType::HashMap
        },
        result
    );
}

#[test]
fn parse_variable_pool_0() {
    let input = "\
1:my_int
2:my_str
1:my_2nd_int
0:my_cool_bool
2:my_2nd_str
3:my_hashmap";

    let result = match <VariablePool as Parsable>::parse(input) {
        Ok(val) => val,
        Err(err) => panic!("Failed to parse variables: {}", err)
    };

    let mut expected_pool = HashMap::<String, Variable>::new();
    expected_pool.insert("my_int".to_string(), Variable { name: "my_int".to_string(), r#type: VariableType::Integer });
    expected_pool.insert("my_str".to_string(), Variable { name: "my_str".to_string(), r#type: VariableType::String });
    expected_pool.insert("my_2nd_int".to_string(), Variable { name: "my_2nd_int".to_string(), r#type: VariableType::Integer });
    expected_pool.insert("my_cool_bool".to_string(), Variable { name: "my_cool_bool".to_string(), r#type: VariableType::Boolean });
    expected_pool.insert("my_2nd_str".to_string(), Variable { name: "my_2nd_str".to_string(), r#type: VariableType::String });
    expected_pool.insert("my_hashmap".to_string(), Variable { name: "my_hashmap".to_string(), r#type: VariableType::HashMap });

    // we can't do assert_eq since hashmap order items randomly
    for variable in expected_pool {
        if let Some(val) = result.0.get(&*variable.0) {
            assert_eq!(*val, variable.1, "In-equal value of variable {}. Result dump: {:?}", variable.0, result)
        } else {
            panic!("Cannot find field {} in the parsed value", variable.0);
        }
    }
}

#[test]
fn parse_variable_pool_1() {
    let input = "\
1:size
1:size2
2:col
2:blacks
2:reds
2:yellows
2:pointer
1:size3
0:v
0:i
2:pinks
2:purple
2:deepPurple
2:Indigo
2:Blue
2:LightBlue
2:cyan
2:teal
2:green
2:lightGreen
2:lime
2:amber
2:Orange
2:deeoOrange
2:brown
2:gray
2:blueGray
2:transparent
2:canvastitle
2:canvasok
2:cwidth
2:cheight
2:canvastoast
1:canvaswidth
1:canvasheight
1:widthpx
1:heightpx
2:canvascancel
1:tv
0:bl_on
0:bl_off
1:h
1:w
1:canvash
1:canvasw
1:hout
1:wout
2:canvas_path
0:lx
0:ly
1:svd
0:vtmundo
2:image_name
2:folder_path
2:output_path";

    let result = match <VariablePool as Parsable>::parse(input) {
        Ok(val) => val,
        Err(err) => panic!("Failed to parse variables: {}", err)
    };

    let mut expected_pool = HashMap::<String, Variable>::new();
    expected_pool.insert("size".to_string(), Variable { name: "size".to_string(), r#type: VariableType::Integer });
    expected_pool.insert("size2".to_string(), Variable { name: "size2".to_string(), r#type: VariableType::Integer });
    expected_pool.insert("col".to_string(), Variable { name: "col".to_string(), r#type: VariableType::String });
    expected_pool.insert("blacks".to_string(), Variable { name: "blacks".to_string(), r#type: VariableType::String });
    expected_pool.insert("reds".to_string(), Variable { name: "reds".to_string(), r#type: VariableType::String });
    expected_pool.insert("yellows".to_string(), Variable { name: "yellows".to_string(), r#type: VariableType::String });
    expected_pool.insert("pointer".to_string(), Variable { name: "pointer".to_string(), r#type: VariableType::String });
    expected_pool.insert("size3".to_string(), Variable { name: "size3".to_string(), r#type: VariableType::Integer });
    expected_pool.insert("v".to_string(), Variable { name: "v".to_string(), r#type: VariableType::Boolean });
    expected_pool.insert("i".to_string(), Variable { name: "i".to_string(), r#type: VariableType::Boolean });
    expected_pool.insert("pinks".to_string(), Variable { name: "pinks".to_string(), r#type: VariableType::String });
    expected_pool.insert("purple".to_string(), Variable { name: "purple".to_string(), r#type: VariableType::String });
    expected_pool.insert("deepPurple".to_string(), Variable { name: "deepPurple".to_string(), r#type: VariableType::String });
    expected_pool.insert("Indigo".to_string(), Variable { name: "Indigo".to_string(), r#type: VariableType::String });
    expected_pool.insert("Blue".to_string(), Variable { name: "Blue".to_string(), r#type: VariableType::String });
    expected_pool.insert("LightBlue".to_string(), Variable { name: "LightBlue".to_string(), r#type: VariableType::String });
    expected_pool.insert("cyan".to_string(), Variable { name: "cyan".to_string(), r#type: VariableType::String });
    expected_pool.insert("teal".to_string(), Variable { name: "teal".to_string(), r#type: VariableType::String });
    expected_pool.insert("green".to_string(), Variable { name: "green".to_string(), r#type: VariableType::String });
    expected_pool.insert("lightGreen".to_string(), Variable { name: "lightGreen".to_string(), r#type: VariableType::String });
    expected_pool.insert("lime".to_string(), Variable { name: "lime".to_string(), r#type: VariableType::String });
    expected_pool.insert("amber".to_string(), Variable { name: "amber".to_string(), r#type: VariableType::String });
    expected_pool.insert("Orange".to_string(), Variable { name: "Orange".to_string(), r#type: VariableType::String });
    expected_pool.insert("deeoOrange".to_string(), Variable { name: "deeoOrange".to_string(), r#type: VariableType::String });
    expected_pool.insert("brown".to_string(), Variable { name: "brown".to_string(), r#type: VariableType::String });
    expected_pool.insert("gray".to_string(), Variable { name: "gray".to_string(), r#type: VariableType::String });
    expected_pool.insert("blueGray".to_string(), Variable { name: "blueGray".to_string(), r#type: VariableType::String });
    expected_pool.insert("transparent".to_string(), Variable { name: "transparent".to_string(), r#type: VariableType::String });
    expected_pool.insert("canvastitle".to_string(), Variable { name: "canvastitle".to_string(), r#type: VariableType::String });
    expected_pool.insert("canvasok".to_string(), Variable { name: "canvasok".to_string(), r#type: VariableType::String });
    expected_pool.insert("cwidth".to_string(), Variable { name: "cwidth".to_string(), r#type: VariableType::String });
    expected_pool.insert("cheight".to_string(), Variable { name: "cheight".to_string(), r#type: VariableType::String });
    expected_pool.insert("canvastoast".to_string(), Variable { name: "canvastoast".to_string(), r#type: VariableType::String });
    expected_pool.insert("canvaswidth".to_string(), Variable { name: "canvaswidth".to_string(), r#type: VariableType::Integer });
    expected_pool.insert("canvasheight".to_string(), Variable { name: "canvasheight".to_string(), r#type: VariableType::Integer });
    expected_pool.insert("widthpx".to_string(), Variable { name: "widthpx".to_string(), r#type: VariableType::Integer });
    expected_pool.insert("heightpx".to_string(), Variable { name: "heightpx".to_string(), r#type: VariableType::Integer });
    expected_pool.insert("canvascancel".to_string(), Variable { name: "canvascancel".to_string(), r#type: VariableType::String });
    expected_pool.insert("tv".to_string(), Variable { name: "tv".to_string(), r#type: VariableType::Integer });
    expected_pool.insert("bl_on".to_string(), Variable { name: "bl_on".to_string(), r#type: VariableType::Boolean });
    expected_pool.insert("bl_off".to_string(), Variable { name: "bl_off".to_string(), r#type: VariableType::Boolean });
    expected_pool.insert("h".to_string(), Variable { name: "h".to_string(), r#type: VariableType::Integer });
    expected_pool.insert("w".to_string(), Variable { name: "w".to_string(), r#type: VariableType::Integer });
    expected_pool.insert("canvash".to_string(), Variable { name: "canvash".to_string(), r#type: VariableType::Integer });
    expected_pool.insert("canvasw".to_string(), Variable { name: "canvasw".to_string(), r#type: VariableType::Integer });
    expected_pool.insert("hout".to_string(), Variable { name: "hout".to_string(), r#type: VariableType::Integer });
    expected_pool.insert("wout".to_string(), Variable { name: "wout".to_string(), r#type: VariableType::Integer });
    expected_pool.insert("canvas_path".to_string(), Variable { name: "canvas_path".to_string(), r#type: VariableType::String });
    expected_pool.insert("lx".to_string(), Variable { name: "lx".to_string(), r#type: VariableType::Boolean });
    expected_pool.insert("ly".to_string(), Variable { name: "ly".to_string(), r#type: VariableType::Boolean });
    expected_pool.insert("svd".to_string(), Variable { name: "svd".to_string(), r#type: VariableType::Integer });
    expected_pool.insert("vtmundo".to_string(), Variable { name: "vtmundo".to_string(), r#type: VariableType::Boolean });
    expected_pool.insert("image_name".to_string(), Variable { name: "image_name".to_string(), r#type: VariableType::String });
    expected_pool.insert("folder_path".to_string(), Variable { name: "folder_path".to_string(), r#type: VariableType::String });
    expected_pool.insert("output_path".to_string(), Variable { name: "output_path".to_string(), r#type: VariableType::String });

    // we can't do assert_eq since hashmap order items randomly
    for variable in expected_pool {
        if let Some(val) = result.0.get(&*variable.0) {
            assert_eq!(*val, variable.1, "In-equal value of variable {}. Result dump: {:?}", variable.0, result)
        } else {
            panic!("Cannot find field {} in the parsed value", variable.0);
        }
    }
}

// Variables =======================================================================================

// List variables ==================================================================================

#[test]
fn parse_list_variable() {
    let input = "0:my_booleans";
    let result = match ListVariable::parse(input) {
        Ok(r) => r,
        Err(err) => panic!("Failed to parse list variable: {}", err)
    };

    assert_eq!(
        result,
        ListVariable {
            name: "my_booleans".to_string(),
            r#type: VariableType::Boolean
        }
    )
}

#[test]
fn parse_list_variable_pool() {
    let input = r#"0:booleans
1:integers
2:strings
3:maps"#;
    let result = match ListVariablePool::parse(input) {
        Ok(r) => r,
        Err(err) => panic!("Failed to parse list variable pool: {}", err)
    };

    let expected = {
        let mut map = HashMap::<String, ListVariable>::new();

        map.insert("booleans".to_string(), ListVariable { name: "booleans".to_string(), r#type: VariableType::Boolean });
        map.insert("integers".to_string(), ListVariable { name: "integers".to_string(), r#type: VariableType::Integer });
        map.insert("strings".to_string(), ListVariable { name: "strings".to_string(), r#type: VariableType::String });
        map.insert("maps".to_string(), ListVariable { name: "maps".to_string(), r#type: VariableType::HashMap });

        ListVariablePool(map)
    };

    assert_eq!(expected, result)
}

// List variables ==================================================================================

// Components ======================================================================================

#[test]
fn parse_component() {
    let input = r#"{"componentId":"dialog","param1":"","param2":"","param3":"","type":7}"#;
    let result = match Component::parse(input) {
        Ok(r) => r,
        Err(err) => panic!("Failed to parse component: {}", err)
    };

    let expected = Component {
        id: "dialog".to_string(),
        param1: "".to_string(),
        param2: "".to_string(),
        param3: "".to_string(),
        r#type: 7
    };

    assert_eq!(result, expected);
}

// Components ======================================================================================

// Events ==========================================================================================

#[test]
fn parse_event() {
    let input = r#"{"eventName":"onClick","eventType":1,"targetId":"linear10","targetType":0}"#;
    let result = match Event::parse(input) {
        Ok(r) => r,
        Err(err) => panic!("Failed to parse event: {}", err)
    };

    let expected = Event {
        event_name: "onClick".to_string(),
        event_type: 1,
        target_id: "linear10".to_string(),
        target_type: 0
    };

    assert_eq!(result, expected);
}

#[test]
fn parse_event_pool() {
    let input = r#"{"eventName":"onResponse","eventType":2,"targetId":"ping_test","targetType":17}
{"eventName":"onErrorResponse","eventType":2,"targetId":"ping_test","targetType":17}
{"eventName":"onClick","eventType":1,"targetId":"item_2","targetType":0}
{"eventName":"onClick","eventType":1,"targetId":"item_3","targetType":0}
{"eventName":"onClick","eventType":1,"targetId":"item_5","targetType":0}
{"eventName":"onClick","eventType":1,"targetId":"item_1","targetType":0}
{"eventName":"onCheckedChange","eventType":1,"targetId":"switch_theme","targetType":13}
{"eventName":"onResponse","eventType":2,"targetId":"get_ip","targetType":17}
{"eventName":"onClick","eventType":1,"targetId":"aver","targetType":0}
{"eventName":"onClick","eventType":1,"targetId":"dsdk","targetType":0}"#;

    let result = match EventPool::parse(input) {
        Ok(r) => r,
        Err(err) => panic!("Failed to parse event pool: {}", err)
    };

    let expected = EventPool(vec![
        Event { event_name: "onResponse".to_string(), event_type: 2, target_id: "ping_test".to_string(), target_type: 17 },
        Event { event_name: "onErrorResponse".to_string(), event_type: 2, target_id: "ping_test".to_string(), target_type: 17 },
        Event { event_name: "onClick".to_string(), event_type: 1, target_id: "item_2".to_string(), target_type: 0 },
        Event { event_name: "onClick".to_string(), event_type: 1, target_id: "item_3".to_string(), target_type: 0 },
        Event { event_name: "onClick".to_string(), event_type: 1, target_id: "item_5".to_string(), target_type: 0 },
        Event { event_name: "onClick".to_string(), event_type: 1, target_id: "item_1".to_string(), target_type: 0 },
        Event { event_name: "onCheckedChange".to_string(), event_type: 1, target_id: "switch_theme".to_string(), target_type: 13 },
        Event { event_name: "onResponse".to_string(), event_type: 2, target_id: "get_ip".to_string(), target_type: 17 },
        Event { event_name: "onClick".to_string(), event_type: 1, target_id: "aver".to_string(), target_type: 0 },
        Event { event_name: "onClick".to_string(), event_type: 1, target_id: "dsdk".to_string(), target_type: 0 },
    ]);

    assert_eq!(result, expected);
}

// Events ==========================================================================================

// Functions / MoreBlocks ==========================================================================

#[test]
fn parse_more_block() {
    let input = "cool_moreblock:hello world %s";
    let result = match MoreBlock::parse(input) {
        Ok(r) => r,
        Err(err) => panic!("Failed to parse moreblock: {}", err)
    };

    let expected = MoreBlock {
        id: "cool_moreblock".to_string(),
        spec: "hello world %s".to_string()
    };

    assert_eq!(result, expected);
}

#[test]
fn parse_more_block_pool_0() {
    let input = "cool_moreblock:hello world %s\nhello_world:very poggers %s cool %i";
    let result = match MoreBlockPool::parse(input) {
        Ok(r) => r,
        Err(err) => panic!("Failed to parse moreblock pool: {}", err)
    };

    let mut expected = HashMap::<String, MoreBlock>::new();
    expected.insert("cool_moreblock".to_string(), MoreBlock { id: "cool_moreblock".to_string(), spec: "hello world %s".to_string() });
    expected.insert("hello_world".to_string(), MoreBlock { id: "hello_world".to_string(), spec: "very poggers %s cool %i".to_string() });

    assert_eq!(result, MoreBlockPool(expected));
}

#[test]
fn parse_more_block_pool_1() {
    let input = "\
CardView:CardView %s.color %d.radius %d.shadow %m.view.view
setBackgroundLinear:setBackgroundLinear %m.view.view fromFilePath %s.path
SaveLinear:SaveLinear %m.view.view to path %s.path
edtDial:edtDial Nazv %s.str1 Mess %s.str2 Hint %s.hint Dialog %m.dialog.dia Output %m.textview.getMess
save_and_exit:save_and_exit Nazv %s.str1 Mess %s.str2 Hint %s.hint Dialog %m.dialog.dia Output %m.textview.getMess
canvas:canvas %d.width %d.height
setCornerRadius:setCornerRadius to %m.view.view percentage %d.percent
EditTextLimit:EditTextLimit %m.textview.edittext setLimit %d.limit output %m.textview.output_tv
saveView:saveView %m.view.view folderPath %s.folderPath outputPath %s.outputPath";

    let result = match MoreBlockPool::parse(input) {
        Ok(r) => r,
        Err(err) => panic!("Failed to parse moreblock pool: {}", err)
    };

    let mut expected = HashMap::<String, MoreBlock>::new();
    expected.insert("CardView".to_string(), MoreBlock { id: "CardView".to_string(), spec: "CardView %s.color %d.radius %d.shadow %m.view.view".to_string() });
    expected.insert("setBackgroundLinear".to_string(), MoreBlock { id: "setBackgroundLinear".to_string(), spec: "setBackgroundLinear %m.view.view fromFilePath %s.path".to_string() });
    expected.insert("SaveLinear".to_string(), MoreBlock { id: "SaveLinear".to_string(), spec: "SaveLinear %m.view.view to path %s.path".to_string() });
    expected.insert("edtDial".to_string(), MoreBlock { id: "edtDial".to_string(), spec: "edtDial Nazv %s.str1 Mess %s.str2 Hint %s.hint Dialog %m.dialog.dia Output %m.textview.getMess".to_string() });
    expected.insert("save_and_exit".to_string(), MoreBlock { id: "save_and_exit".to_string(), spec: "save_and_exit Nazv %s.str1 Mess %s.str2 Hint %s.hint Dialog %m.dialog.dia Output %m.textview.getMess".to_string() });
    expected.insert("canvas".to_string(), MoreBlock { id: "canvas".to_string(), spec: "canvas %d.width %d.height".to_string() });
    expected.insert("setCornerRadius".to_string(), MoreBlock { id: "setCornerRadius".to_string(), spec: "setCornerRadius to %m.view.view percentage %d.percent".to_string() });
    expected.insert("EditTextLimit".to_string(), MoreBlock { id: "EditTextLimit".to_string(), spec: "EditTextLimit %m.textview.edittext setLimit %d.limit output %m.textview.output_tv".to_string() });
    expected.insert("saveView".to_string(), MoreBlock { id: "saveView".to_string(), spec: "saveView %m.view.view folderPath %s.folderPath outputPath %s.outputPath".to_string() });

    assert_eq!(result, MoreBlockPool(expected));
}

// Functions / MoreBlocks ==========================================================================

// Full logic ======================================================================================

#[test]
fn parse_logic_0() {
    // A project of mine
    let input = r#"@MainActivity.java_var
2:out

@MainActivity.java_func
execute_shell:execute_shell %s.command

@MainActivity.java_onCreate_initializeLogic
{"color":-10701022,"id":"12","nextBlock":10,"opCode":"addSourceDirectly","parameters":["// For those who are familiar with Linux commands, we\u0027re reading /proc/cpuinfo, and /proc/meminfo"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"10","nextBlock":13,"opCode":"definedFunc","parameters":["cat /proc/cpuinfo"],"spec":"execute_shell %s.command","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"13","nextBlock":11,"opCode":"setText","parameters":["cpuinfo","@16"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-1147626,"id":"16","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"out","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-7711273,"id":"11","nextBlock":14,"opCode":"definedFunc","parameters":["cat /proc/meminfo"],"spec":"execute_shell %s.command","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"14","nextBlock":-1,"opCode":"setText","parameters":["raminfo","@15"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-1147626,"id":"15","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"out","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}

@MainActivity.java_execute_shell_moreBlock
{"color":-10701022,"id":"10","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["StringBuilder output \u003d new StringBuilder();\ntry {\njava.lang.Process cmdProc \u003d Runtime.getRuntime().exec(_command);\n\n\njava.io.BufferedReader stdoutReader \u003d new java.io.BufferedReader(\n         new java.io.InputStreamReader(cmdProc.getInputStream()));\nString line;\nwhile ((line \u003d stdoutReader.readLine()) !\u003d null) {\n   // process procs standard output here\n  output.append(line + \"\\n\");\n}\n\nthis.out \u003d output.toString();\n\n} catch (java.io.IOException e) {\nthis.out \u003d \"Error occurred\";\n}"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}"#;

    match Logic::parse(input) {
        Ok(r) => r,
        Err(err) => panic!("Failed to parse logic: {}", err)
    };

    // sadly i can't check if it produces a correct output, if someone got time, please do this
}

#[test]
fn parse_logic_1() {
    // An incredible sketchware project made by NotteShock (formerly "7Studio") on Sketchub named "7DeviceInfo" (yes he allowed me to use this)
    let input = r##"@DeviceinfoActivity.java_var
2:api
2:release
2:manufacturer
2:model
2:fingerprint
2:arch
2:bootloader
2:kernel
2:out
0:start
1:ping
3:get
1:mem_max
1:mem_usage
1:count
3:info
2:details
3:lat
3:lon
1:spin
3:country
2:CPUABI

@MainActivity.java_var
2:api
2:release
2:manufacturer
2:model
2:kernel
2:bootloader
2:fingerprint
2:arch
3:ip_address
0:start
1:ping
2:out

@SpeakerCleanerActivity.java_var
2:useless

@DeviceinfoActivity.java_func
execute_shell:execute_shell %s.command
generalDeviceInfo:generalDeviceInfo
Level:Level
Health:Health
Status:Status
Temperature:Temperature
ping_start:ping_start
LightStatusBar:LightStatusBar
GUIFont:GUIFont
Round:Round %m.view.view Color1 %s.color1 Color2 %s.color2
RoundAndBorder:RoundAndBorder %m.view.view backColor %s.color1 border %d.border borderColor %s.color2 round %d.round
clickAnimation:clickAnimation %m.view.view
copyToClipboard:copyToClipboard %m.textview.textview
status_bar_color:status_bar_color %s.colour1 bottom_navigation_bar %s.colour2
Elevation:Elevation %m.view.view set %d.number
Animator:Animator %m.view.view %s.propertyName %d.value %d.duration
drawer_light:drawer_light
drawer_dark:drawer_dark
Selectable_Textview:Selectable_Textview %m.textview.view
DarkUI:DarkUI
LightUI:LightUI
drawerFont:drawerFont
ram_progress:ram_progress

@MainActivity.java_func
lscaleY:lscaleY
imageScale:imageScale
rippleRoundStroke:rippleRoundStroke %m.view.view onFocus %s.focus onPressed %s.pressed setRound %d.round setStroke %d.stroke setStrokeColor %s.strokeclr
Elevation:Elevation %m.view.view set %d.number
status_bar_color:status_bar_color %s.colour1 bottom_navigation_bar %s.colour2
logic:logic

@DisplayTestActivity.java_func
Background:Background setType %s.type

@SpeakerCleanerActivity.java_func
LightStatusBar:LightStatusBar
font:font
DarkUI:DarkUI
LightUI:LightUI
status_bar_color:status_bar_color %s.colour1 bottom_navigation_bar %s.colour2
Round:Round %m.view.view Color1 %s.color1 Color2 %s.color2

@AboutActivity.java_func
LightStatusBar:LightStatusBar
status_bar_color:status_bar_color %s.colour1 bottom_navigation_bar %s.colour2
Round:Round %m.view.view Color1 %s.color1 Color2 %s.color2
RoundAndBorder:RoundAndBorder %m.view.view backColor %s.color1 border %d.border borderColor %s.color2 round %d.round
DarkUI:DarkUI
LightUI:LightUI
ClickAnimation:ClickAnimation %b.clickanim Duration %d.animDuration %m.view.view
Send:Send Email %m.intent.IntentName To %s.to subject %s.subject text %s.text
contact_me_theme:contact_me_theme %d.theme
round_contact_me:round_contact_me %m.view.view Color1 %s.color1 Color2 %s.color2

@ToolsActivity.java_func
DarkUI:DarkUI
LightUI:LightUI
font:font
LightStatusBar:LightStatusBar
status_bar_color:status_bar_color %s.colour1 bottom_navigation_bar %s.colour2
Round:Round %m.view.view Color1 %s.color1 Color2 %s.color2
ClickAnimation:ClickAnimation %b.clickanim Duration %d.animDuration %m.view.view

@DeviceinfoActivity.java_components
{"componentId":"healthbc","param1":"","param2":"","param3":"","type":1}
{"componentId":"ping_timer","param1":"","param2":"","param3":"","type":5}
{"componentId":"ping_test","param1":"","param2":"","param3":"","type":17}
{"componentId":"get_ip","param1":"","param2":"","param3":"","type":17}
{"componentId":"timer","param1":"","param2":"","param3":"","type":5}
{"componentId":"theme","param1":"theme","param2":"","param3":"","type":2}
{"componentId":"change_log","param1":"","param2":"","param3":"","type":7}
{"componentId":"changelog","param1":"changelog","param2":"","param3":"","type":2}
{"componentId":"go_to","param1":"","param2":"","param3":"","type":1}
{"componentId":"themedata","param1":"themedata","param2":"","param3":"","type":2}
{"componentId":"date_time","param1":"","param2":"","param3":"","type":3}
{"componentId":"instructions","param1":"","param2":"","param3":"","type":7}

@MainActivity.java_components
{"componentId":"object","param1":"","param2":"","param3":"","type":10}
{"componentId":"c","param1":"","param2":"","param3":"","type":10}
{"componentId":"b","param1":"","param2":"","param3":"","type":10}
{"componentId":"d","param1":"","param2":"","param3":"","type":10}
{"componentId":"bd","param1":"","param2":"","param3":"","type":10}
{"componentId":"go_to","param1":"","param2":"","param3":"","type":1}
{"componentId":"timer","param1":"","param2":"","param3":"","type":5}

@DisplayTestActivity.java_components
{"componentId":"time_switch","param1":"","param2":"","param3":"","type":5}
{"componentId":"warning","param1":"","param2":"","param3":"","type":7}
{"componentId":"exit","param1":"","param2":"","param3":"","type":1}

@SpeakerCleanerActivity.java_components
{"componentId":"cleaner","param1":"","param2":"","param3":"","type":8}
{"componentId":"timer","param1":"","param2":"","param3":"","type":5}
{"componentId":"back","param1":"","param2":"","param3":"","type":1}

@AboutActivity.java_components
{"componentId":"Link","param1":"","param2":"","param3":"","type":1}

@ToolsActivity.java_components
{"componentId":"go","param1":"","param2":"","param3":"","type":1}

@DeviceinfoActivity.java_events
{"eventName":"onResponse","eventType":2,"targetId":"ping_test","targetType":17}
{"eventName":"onErrorResponse","eventType":2,"targetId":"ping_test","targetType":17}
{"eventName":"onClick","eventType":1,"targetId":"item_2","targetType":0}
{"eventName":"onClick","eventType":1,"targetId":"item_3","targetType":0}
{"eventName":"onClick","eventType":1,"targetId":"item_5","targetType":0}
{"eventName":"onClick","eventType":1,"targetId":"item_1","targetType":0}
{"eventName":"onCheckedChange","eventType":1,"targetId":"switch_theme","targetType":13}
{"eventName":"onResponse","eventType":2,"targetId":"get_ip","targetType":17}
{"eventName":"onClick","eventType":1,"targetId":"aver","targetType":0}
{"eventName":"onClick","eventType":1,"targetId":"dsdk","targetType":0}
{"eventName":"onClick","eventType":1,"targetId":"dmam","targetType":0}
{"eventName":"onClick","eventType":1,"targetId":"dbk","targetType":0}
{"eventName":"onClick","eventType":1,"targetId":"df","targetType":0}
{"eventName":"onClick","eventType":1,"targetId":"ip","targetType":0}
{"eventName":"onClick","eventType":1,"targetId":"background","targetType":0}
{"eventName":"onClick","eventType":1,"targetId":"bh","targetType":0}
{"eventName":"onClick","eventType":1,"targetId":"btemp","targetType":0}
{"eventName":"onClick","eventType":1,"targetId":"bstatus","targetType":0}
{"eventName":"onClick","eventType":1,"targetId":"cpui","targetType":0}
{"eventName":"onClick","eventType":1,"targetId":"rami","targetType":0}
{"eventName":"onErrorResponse","eventType":2,"targetId":"get_ip","targetType":17}
{"eventName":"onBackPressed","eventType":3,"targetId":"onBackPressed","targetType":0}
{"eventName":"onClick","eventType":1,"targetId":"item_4","targetType":0}
{"eventName":"onClick","eventType":4,"targetId":"linear_home","targetType":0}
{"eventName":"onClick","eventType":4,"targetId":"linear_aboutapp","targetType":0}
{"eventName":"onClick","eventType":4,"targetId":"linear_about","targetType":0}
{"eventName":"onClick","eventType":4,"targetId":"linear_support","targetType":0}
{"eventName":"onClick","eventType":4,"targetId":"linear_rate","targetType":0}
{"eventName":"onClick","eventType":1,"targetId":"title","targetType":4}
{"eventName":"onClick","eventType":1,"targetId":"linear7","targetType":0}
{"eventName":"onClick","eventType":1,"targetId":"linear8","targetType":0}
{"eventName":"onClick","eventType":4,"targetId":"aboutt","targetType":0}
{"eventName":"onClick","eventType":1,"targetId":"isp","targetType":0}
{"eventName":"onClick","eventType":4,"targetId":"close","targetType":6}
{"eventName":"onClick","eventType":4,"targetId":"fix","targetType":0}
{"eventName":"onClick","eventType":1,"targetId":"linear9","targetType":0}
{"eventName":"onClick","eventType":1,"targetId":"linear10","targetType":0}
{"eventName":"onClick","eventType":1,"targetId":"linear12","targetType":0}

@MainActivity.java_events
{"eventName":"onResume","eventType":3,"targetId":"onResume","targetType":0}

@SpeakerCleanerActivity.java_events
{"eventName":"onBackPressed","eventType":3,"targetId":"onBackPressed","targetType":0}
{"eventName":"onPause","eventType":3,"targetId":"onPause","targetType":0}
{"eventName":"onResume","eventType":3,"targetId":"onResume","targetType":0}

@AboutActivity.java_events
{"eventName":"onClick","eventType":1,"targetId":"whatsapp","targetType":6}
{"eventName":"onClick","eventType":1,"targetId":"facebook","targetType":6}
{"eventName":"onClick","eventType":1,"targetId":"email","targetType":6}

@ToolsActivity.java_events
{"eventName":"onClick","eventType":1,"targetId":"linear7","targetType":0}
{"eventName":"onClick","eventType":1,"targetId":"linear10","targetType":0}

@DeviceinfoActivity.java_GUIFont_moreBlock
{"color":-11899692,"id":"10","nextBlock":11,"opCode":"setTypeface","parameters":["title","font_light",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"11","nextBlock":12,"opCode":"setTypeface","parameters":["title_android_ver","font_light",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"12","nextBlock":13,"opCode":"setTypeface","parameters":["title_device_sdk","font_light",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"13","nextBlock":14,"opCode":"setTypeface","parameters":["title_dev_manu_model","font_light",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"14","nextBlock":15,"opCode":"setTypeface","parameters":["dbk_title","font_light",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"15","nextBlock":16,"opCode":"setTypeface","parameters":["title_device_fp_arch","font_light",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"16","nextBlock":17,"opCode":"setTypeface","parameters":["title_root_status","font_light",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"17","nextBlock":18,"opCode":"setTypeface","parameters":["title_ip_address","font_light",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"18","nextBlock":19,"opCode":"setTypeface","parameters":["title_ping","font_light",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"19","nextBlock":20,"opCode":"setTypeface","parameters":["title_b_level","font_light",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"20","nextBlock":21,"opCode":"setTypeface","parameters":["title_b_health","font_light",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"21","nextBlock":22,"opCode":"setTypeface","parameters":["title_b_temp","font_light",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"22","nextBlock":23,"opCode":"setTypeface","parameters":["title_b_status","font_light",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"23","nextBlock":24,"opCode":"setTypeface","parameters":["title_cpu_info","font_light",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"24","nextBlock":25,"opCode":"setTypeface","parameters":["title_ram_info","font_light",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"25","nextBlock":42,"opCode":"setTypeface","parameters":["isp_title","font_light",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"42","nextBlock":44,"opCode":"setTypeface","parameters":["title_ram_usage","font_light",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"44","nextBlock":46,"opCode":"setTypeface","parameters":["fdi_title","font_light",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"46","nextBlock":48,"opCode":"setTypeface","parameters":["country_title","font_light",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"48","nextBlock":50,"opCode":"setTypeface","parameters":["title_cpu_abi","font_light",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"50","nextBlock":52,"opCode":"setTypeface","parameters":["title_screen_size","font_light",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"52","nextBlock":26,"opCode":"setTypeface","parameters":["date_time_title","font_light",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"26","nextBlock":27,"opCode":"setTypeface","parameters":["switch_theme","font_light",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"27","nextBlock":28,"opCode":"setTypeface","parameters":["show_android_version","font_regular",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"28","nextBlock":29,"opCode":"setTypeface","parameters":["show_device_sdk","font_regular",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"29","nextBlock":30,"opCode":"setTypeface","parameters":["manufacturer_model","font_regular",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"30","nextBlock":31,"opCode":"setTypeface","parameters":["device_bootloader_k","font_regular",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"31","nextBlock":32,"opCode":"setTypeface","parameters":["show_fp_arch","font_regular",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"32","nextBlock":33,"opCode":"setTypeface","parameters":["show_root_status","font_regular",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"33","nextBlock":34,"opCode":"setTypeface","parameters":["show_ip_address","font_regular",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"34","nextBlock":35,"opCode":"setTypeface","parameters":["ping_view","font_regular",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"35","nextBlock":36,"opCode":"setTypeface","parameters":["show_b_level","font_regular",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"36","nextBlock":37,"opCode":"setTypeface","parameters":["battery_health","font_regular",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"37","nextBlock":38,"opCode":"setTypeface","parameters":["battery_temperature","font_regular",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"38","nextBlock":39,"opCode":"setTypeface","parameters":["statust","font_regular",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"39","nextBlock":40,"opCode":"setTypeface","parameters":["show_cpu_info","font_regular",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"40","nextBlock":43,"opCode":"setTypeface","parameters":["show_ram_info","font_regular",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"43","nextBlock":41,"opCode":"setTypeface","parameters":["show_ram_usage","font_regular",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"41","nextBlock":45,"opCode":"setTypeface","parameters":["show_isp_name","font_regular",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"45","nextBlock":49,"opCode":"setTypeface","parameters":["show_fdi","font_regular",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"49","nextBlock":51,"opCode":"setTypeface","parameters":["cpu_abi_show","font_regular",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"51","nextBlock":53,"opCode":"setTypeface","parameters":["screen_size_show","font_regular",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"53","nextBlock":47,"opCode":"setTypeface","parameters":["date_time_show","font_regular",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"47","nextBlock":-1,"opCode":"setTypeface","parameters":["country_show","font_regular",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_Selectable_Textview_moreBlock
{"color":-10701022,"id":"10","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["_view.setTextIsSelectable(true);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_isp_onClick
{"color":-7711273,"id":"10","nextBlock":-1,"opCode":"definedFunc","parameters":["show_isp_name"],"spec":"copyToClipboard %m.textview.textview","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_item_5_onClick
{"color":-11899692,"id":"20","nextBlock":21,"opCode":"setText","parameters":["title","Settings"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"21","nextBlock":22,"opCode":"setVisible","parameters":["general_info_main","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"22","nextBlock":23,"opCode":"setVisible","parameters":["battery_info_main","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"23","nextBlock":24,"opCode":"setVisible","parameters":["cpu_ram_info_main","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"24","nextBlock":25,"opCode":"setVisible","parameters":["settings_main","VISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"25","nextBlock":26,"opCode":"setVisible","parameters":["network_info_main","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"26","nextBlock":27,"opCode":"setVisible","parameters":["dock1","INVISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"27","nextBlock":28,"opCode":"setVisible","parameters":["dock2","INVISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"28","nextBlock":29,"opCode":"setVisible","parameters":["dock3","INVISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"29","nextBlock":30,"opCode":"setVisible","parameters":["dock4","INVISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"30","nextBlock":31,"opCode":"setVisible","parameters":["dock5","VISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"31","nextBlock":-1,"opCode":"definedFunc","parameters":["i_5"],"spec":"clickAnimation %m.view.view","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_Status_moreBlock
{"color":-10701022,"id":"10","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["IntentFilter bstatus \u003d new IntentFilter(Intent.ACTION_BATTERY_CHANGED);\n\nIntent batteryStatus \u003d registerReceiver(null, bstatus);\n\nint chargePlug \u003d batteryStatus.getIntExtra(BatteryManager.EXTRA_PLUGGED,-1);\n\nboolean usbCharge \u003d chargePlug \u003d\u003d BatteryManager.BATTERY_PLUGGED_USB;\n\nboolean acCharge \u003d chargePlug \u003d\u003d BatteryManager.BATTERY_PLUGGED_AC;\n\nif(usbCharge){\n\nstatust.setText(\"Device charging through USB\");\n\n} else if(acCharge) {\n\nstatust.setText(\"Device charging through AC\" );\n\n} else {\n\nstatust.setText(\"Device is not charging\");\n\n}\n\n"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_item_1_onClick
{"color":-11899692,"id":"22","nextBlock":10,"opCode":"setText","parameters":["title","General info"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"10","nextBlock":11,"opCode":"setVisible","parameters":["general_info_main","VISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"11","nextBlock":12,"opCode":"setVisible","parameters":["battery_info_main","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"12","nextBlock":13,"opCode":"setVisible","parameters":["cpu_ram_info_main","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"13","nextBlock":24,"opCode":"setVisible","parameters":["settings_main","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"24","nextBlock":18,"opCode":"setVisible","parameters":["network_info_main","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"18","nextBlock":19,"opCode":"setVisible","parameters":["dock1","VISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"19","nextBlock":20,"opCode":"setVisible","parameters":["dock2","INVISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"20","nextBlock":21,"opCode":"setVisible","parameters":["dock3","INVISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"21","nextBlock":25,"opCode":"setVisible","parameters":["dock4","INVISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"25","nextBlock":23,"opCode":"setVisible","parameters":["dock5","INVISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"23","nextBlock":-1,"opCode":"definedFunc","parameters":["i_1"],"spec":"clickAnimation %m.view.view","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_ping_test_onResponse
{"color":-1147626,"id":"10","nextBlock":12,"opCode":"setVarBoolean","parameters":["start","@11"],"spec":"set %m.varBool to %b","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"11","nextBlock":-1,"opCode":"false","parameters":[],"spec":"false","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-11899692,"id":"12","nextBlock":16,"opCode":"setText","parameters":["ping_view","@13"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"13","nextBlock":-1,"opCode":"stringJoin","parameters":["@14"," ms"],"spec":"join %s and %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-10701022,"id":"14","nextBlock":-1,"opCode":"toString","parameters":["@15"],"spec":"toString %d without decimal","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-1147626,"id":"15","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"ping","subStack1":-1,"subStack2":-1,"type":"d","typeName":""}
{"color":-13851166,"id":"16","nextBlock":-1,"opCode":"timerAfter","parameters":["ping_timer","500"],"spec":"%m.timer after %d ms","subStack1":17,"subStack2":-1,"type":"c","typeName":""}
{"color":-1147626,"id":"17","nextBlock":18,"opCode":"setVarInt","parameters":["ping","0"],"spec":"set %m.varInt to %d","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-1147626,"id":"18","nextBlock":20,"opCode":"setVarBoolean","parameters":["start","@19"],"spec":"set %m.varBool to %b","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"19","nextBlock":-1,"opCode":"true","parameters":[],"spec":"true","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-7711273,"id":"20","nextBlock":21,"opCode":"definedFunc","parameters":[],"spec":"ping_start","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"21","nextBlock":-1,"opCode":"requestnetworkStartRequestNetwork","parameters":["ping_test","GET","https://www.google.com/","A"],"spec":"%m.requestnetwork start network request to method %m.method to url %s with tag %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_settings_onClick
{"color":-11899692,"id":"22","nextBlock":10,"opCode":"closeDrawer","parameters":[],"spec":"closeDrawer","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"10","nextBlock":11,"opCode":"setText","parameters":["title","Settings"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"11","nextBlock":12,"opCode":"setVisible","parameters":["general_info_main","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"12","nextBlock":13,"opCode":"setVisible","parameters":["battery_info_main","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"13","nextBlock":14,"opCode":"setVisible","parameters":["cpu_ram_info_main","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"14","nextBlock":15,"opCode":"setVisible","parameters":["settings_main","VISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"15","nextBlock":16,"opCode":"setVisible","parameters":["network_info_main","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"16","nextBlock":17,"opCode":"setVisible","parameters":["dock1","INVISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"17","nextBlock":18,"opCode":"setVisible","parameters":["dock2","INVISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"18","nextBlock":19,"opCode":"setVisible","parameters":["dock3","INVISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"19","nextBlock":20,"opCode":"setVisible","parameters":["dock4","INVISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"20","nextBlock":-1,"opCode":"setVisible","parameters":["dock5","VISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_LightStatusBar_moreBlock
{"color":-10701022,"id":"10","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["getWindow().getDecorView().setSystemUiVisibility(View.SYSTEM_UI_FLAG_LIGHT_STATUS_BAR);\r\ngetWindow().setStatusBarColor(0xFFFFFFFF);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_linear_aboutapp_onClick
{"color":-11899692,"id":"21","nextBlock":10,"opCode":"closeDrawer","parameters":[],"spec":"closeDrawer","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"10","nextBlock":11,"opCode":"setText","parameters":["title","Battery info"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"11","nextBlock":12,"opCode":"setVisible","parameters":["general_info_main","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"12","nextBlock":13,"opCode":"setVisible","parameters":["battery_info_main","VISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"13","nextBlock":14,"opCode":"setVisible","parameters":["cpu_ram_info_main","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"14","nextBlock":15,"opCode":"setVisible","parameters":["settings_main","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"15","nextBlock":16,"opCode":"setVisible","parameters":["network_info_main","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"16","nextBlock":17,"opCode":"setVisible","parameters":["dock1","INVISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"17","nextBlock":18,"opCode":"setVisible","parameters":["dock2","VISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"18","nextBlock":19,"opCode":"setVisible","parameters":["dock3","INVISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"19","nextBlock":20,"opCode":"setVisible","parameters":["dock4","INVISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"20","nextBlock":-1,"opCode":"setVisible","parameters":["dock5","INVISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_linear_support_onClick
{"color":-11899692,"id":"21","nextBlock":10,"opCode":"closeDrawer","parameters":[],"spec":"closeDrawer","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"10","nextBlock":11,"opCode":"setText","parameters":["title","Network info"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"11","nextBlock":12,"opCode":"setVisible","parameters":["general_info_main","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"12","nextBlock":13,"opCode":"setVisible","parameters":["battery_info_main","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"13","nextBlock":14,"opCode":"setVisible","parameters":["cpu_ram_info_main","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"14","nextBlock":15,"opCode":"setVisible","parameters":["settings_main","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"15","nextBlock":16,"opCode":"setVisible","parameters":["network_info_main","VISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"16","nextBlock":17,"opCode":"setVisible","parameters":["dock1","INVISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"17","nextBlock":18,"opCode":"setVisible","parameters":["dock2","INVISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"18","nextBlock":19,"opCode":"setVisible","parameters":["dock3","INVISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"19","nextBlock":20,"opCode":"setVisible","parameters":["dock4","VISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"20","nextBlock":-1,"opCode":"setVisible","parameters":["dock5","INVISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_bh_onClick
{"color":-7711273,"id":"10","nextBlock":-1,"opCode":"definedFunc","parameters":["battery_health"],"spec":"copyToClipboard %m.textview.textview","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_item_3_onClick
{"color":-11899692,"id":"20","nextBlock":21,"opCode":"setText","parameters":["title","CPU \u0026 RAM info"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"21","nextBlock":22,"opCode":"setVisible","parameters":["general_info_main","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"22","nextBlock":23,"opCode":"setVisible","parameters":["battery_info_main","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"23","nextBlock":24,"opCode":"setVisible","parameters":["cpu_ram_info_main","VISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"24","nextBlock":25,"opCode":"setVisible","parameters":["settings_main","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"25","nextBlock":26,"opCode":"setVisible","parameters":["network_info_main","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"26","nextBlock":27,"opCode":"setVisible","parameters":["dock1","INVISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"27","nextBlock":28,"opCode":"setVisible","parameters":["dock2","INVISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"28","nextBlock":29,"opCode":"setVisible","parameters":["dock3","VISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"29","nextBlock":30,"opCode":"setVisible","parameters":["dock4","INVISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"30","nextBlock":31,"opCode":"setVisible","parameters":["dock5","INVISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"31","nextBlock":-1,"opCode":"definedFunc","parameters":["i_3"],"spec":"clickAnimation %m.view.view","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_Round_moreBlock
{"color":-10701022,"id":"10","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["int[] colors \u003d { Color.parseColor(_color1), Color.parseColor(_color2) }; android.graphics.drawable.GradientDrawable gd \u003d new android.graphics.drawable.GradientDrawable(android.graphics.drawable.GradientDrawable.Orientation.RIGHT_LEFT, colors); gd.setCornerRadius(999); _view.setBackground(gd);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_close_onClick
{"color":-11899692,"id":"10","nextBlock":-1,"opCode":"closeDrawer","parameters":[],"spec":"closeDrawer","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_linear_about_onClick
{"color":-11899692,"id":"21","nextBlock":10,"opCode":"closeDrawer","parameters":[],"spec":"closeDrawer","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"10","nextBlock":11,"opCode":"setText","parameters":["title","CPU \u0026 RAM info"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"11","nextBlock":12,"opCode":"setVisible","parameters":["general_info_main","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"12","nextBlock":13,"opCode":"setVisible","parameters":["battery_info_main","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"13","nextBlock":14,"opCode":"setVisible","parameters":["cpu_ram_info_main","VISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"14","nextBlock":15,"opCode":"setVisible","parameters":["settings_main","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"15","nextBlock":16,"opCode":"setVisible","parameters":["network_info_main","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"16","nextBlock":17,"opCode":"setVisible","parameters":["dock1","INVISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"17","nextBlock":18,"opCode":"setVisible","parameters":["dock2","INVISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"18","nextBlock":19,"opCode":"setVisible","parameters":["dock3","VISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"19","nextBlock":20,"opCode":"setVisible","parameters":["dock4","INVISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"20","nextBlock":-1,"opCode":"setVisible","parameters":["dock5","INVISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_rami_onClick
{"color":-7711273,"id":"10","nextBlock":-1,"opCode":"definedFunc","parameters":["show_ram_info"],"spec":"copyToClipboard %m.textview.textview","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_linear9_onClick
{"color":-7711273,"id":"10","nextBlock":-1,"opCode":"definedFunc","parameters":["cpu_abi_show"],"spec":"Selectable_Textview %m.textview.view","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_linear10_onClick
{"color":-7711273,"id":"10","nextBlock":-1,"opCode":"definedFunc","parameters":["screen_size_show"],"spec":"copyToClipboard %m.textview.textview","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_drawerFont_moreBlock
{"color":-10701022,"id":"10","nextBlock":11,"opCode":"addSourceDirectly","parameters":["_drawer_title.setTypeface(Typeface.createFromAsset(getAssets(),\n\"fonts/font_light.ttf\"), 0);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"11","nextBlock":12,"opCode":"addSourceDirectly","parameters":["_drawer_subtitle.setTypeface(Typeface.createFromAsset(getAssets(),\n\"fonts/font_light.ttf\"), 0);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"12","nextBlock":13,"opCode":"addSourceDirectly","parameters":["_drawer_general.setTypeface(Typeface.createFromAsset(getAssets(),\n\"fonts/font_light.ttf\"), 0);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"13","nextBlock":14,"opCode":"addSourceDirectly","parameters":["_drawer_battery.setTypeface(Typeface.createFromAsset(getAssets(),\n\"fonts/font_light.ttf\"), 0);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"14","nextBlock":15,"opCode":"addSourceDirectly","parameters":["_drawer_cpuaram.setTypeface(Typeface.createFromAsset(getAssets(),\n\"fonts/font_light.ttf\"), 0);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"15","nextBlock":16,"opCode":"addSourceDirectly","parameters":["_drawer_network.setTypeface(Typeface.createFromAsset(getAssets(),\n\"fonts/font_light.ttf\"), 0);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"16","nextBlock":18,"opCode":"addSourceDirectly","parameters":["_drawer_sett.setTypeface(Typeface.createFromAsset(getAssets(),\n\"fonts/font_light.ttf\"), 0);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"18","nextBlock":17,"opCode":"addSourceDirectly","parameters":["_drawer_fix_device.setTypeface(Typeface.createFromAsset(getAssets(),\n\"fonts/font_light.ttf\"), 0);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"17","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["_drawer_about.setTypeface(Typeface.createFromAsset(getAssets(),\n\"fonts/font_light.ttf\"), 0);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_linear12_onClick
{"color":-7711273,"id":"10","nextBlock":-1,"opCode":"definedFunc","parameters":["date_time_show"],"spec":"copyToClipboard %m.textview.textview","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_ping_test_onErrorResponse
{"color":-1147626,"id":"10","nextBlock":12,"opCode":"setVarBoolean","parameters":["start","@11"],"spec":"set %m.varBool to %b","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"11","nextBlock":-1,"opCode":"true","parameters":[],"spec":"true","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-13851166,"id":"12","nextBlock":13,"opCode":"requestnetworkStartRequestNetwork","parameters":["ping_test","GET","https://www.google.com/","A"],"spec":"%m.requestnetwork start network request to method %m.method to url %s with tag %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"13","nextBlock":-1,"opCode":"setText","parameters":["ping_view","---- ms"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_dmam_onClick
{"color":-7711273,"id":"10","nextBlock":-1,"opCode":"definedFunc","parameters":["manufacturer_model"],"spec":"copyToClipboard %m.textview.textview","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_Elevation_moreBlock
{"color":-10701022,"id":"10","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["\n_view.setElevation((int)_number);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_linear_rate_onClick
{"color":-11899692,"id":"21","nextBlock":10,"opCode":"closeDrawer","parameters":[],"spec":"closeDrawer","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"10","nextBlock":11,"opCode":"setText","parameters":["title","Settings"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"11","nextBlock":12,"opCode":"setVisible","parameters":["general_info_main","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"12","nextBlock":13,"opCode":"setVisible","parameters":["battery_info_main","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"13","nextBlock":14,"opCode":"setVisible","parameters":["cpu_ram_info_main","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"14","nextBlock":15,"opCode":"setVisible","parameters":["settings_main","VISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"15","nextBlock":16,"opCode":"setVisible","parameters":["network_info_main","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"16","nextBlock":17,"opCode":"setVisible","parameters":["dock1","INVISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"17","nextBlock":18,"opCode":"setVisible","parameters":["dock2","INVISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"18","nextBlock":19,"opCode":"setVisible","parameters":["dock3","INVISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"19","nextBlock":20,"opCode":"setVisible","parameters":["dock4","INVISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"20","nextBlock":-1,"opCode":"setVisible","parameters":["dock5","VISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_linear7_onClick
{"color":-7711273,"id":"10","nextBlock":-1,"opCode":"definedFunc","parameters":["show_fdi"],"spec":"copyToClipboard %m.textview.textview","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_get_ip_onErrorResponse
{"color":-1988310,"id":"103","nextBlock":128,"opCode":"if","parameters":["@104"],"spec":"if %b then","subStack1":147,"subStack2":-1,"type":"c","typeName":""}
{"color":-10701022,"id":"104","nextBlock":-1,"opCode":"stringEquals","parameters":["@105","light"],"spec":"%s equals %s","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-13851166,"id":"105","nextBlock":-1,"opCode":"fileGetData","parameters":["theme","theme"],"spec":"%m.file getData key %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-10701022,"id":"147","nextBlock":148,"opCode":"addSourceDirectly","parameters":["LayoutInflater Inflater \u003d getLayoutInflater();\n\nView InfView \u003d getLayoutInflater().inflate(R.layout.design_3, null);\n\nLinearLayout border \u003d (LinearLayout) InfView.findViewById(R.id.border);\n\nLinearLayout line_content \u003d (LinearLayout) InfView.findViewById(R.id.line_content);\n\nLinearLayout line \u003d (LinearLayout) InfView.findViewById (R.id.line);\n\nTextView title \u003d (TextView) InfView.findViewById(R.id.title);\n\nTextView content \u003d (TextView) InfView.findViewById(R.id.content);\n\nImageView image_logo \u003d (ImageView) InfView.findViewById(R.id.image_logo);\n\nToast ToastName \u003d Toast.makeText(getApplicationContext(),\"\",Toast.LENGTH_SHORT);\n\nToastName.setView(InfView);\n\nToastName.show();"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"148","nextBlock":149,"opCode":"addSourceDirectly","parameters":["image_logo.setImageResource(R.drawable.warn);\nborder.setElevation((float)2);\ntitle.setTextColor(Color.parseColor(\"#47D765\"));"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"149","nextBlock":151,"opCode":"setTextColor","parameters":["@150","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"150","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["title"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":"v","typeName":"TextView"}
{"color":-11899692,"id":"151","nextBlock":153,"opCode":"setTypeface","parameters":["@152","font_light","bold"],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"152","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["title"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":"v","typeName":"TextView"}
{"color":-11899692,"id":"153","nextBlock":155,"opCode":"setTypeface","parameters":["@154","font_regular","normal"],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"154","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["content"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":"v","typeName":"TextView"}
{"color":-7711273,"id":"155","nextBlock":157,"opCode":"definedFunc","parameters":["@156","#ffffff","0","#00000000","8"],"spec":"RoundAndBorder %m.view.view backColor %s.color1 border %d.border borderColor %s.color2 round %d.round","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"156","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["border"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":"v","typeName":"TextView"}
{"color":-7711273,"id":"157","nextBlock":159,"opCode":"definedFunc","parameters":["@158","#FFC122","0","#00000000","360"],"spec":"RoundAndBorder %m.view.view backColor %s.color1 border %d.border borderColor %s.color2 round %d.round","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"158","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["line"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":"v","typeName":"TextView"}
{"color":-11899692,"id":"159","nextBlock":161,"opCode":"setText","parameters":["@160","No internet connection!"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"160","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["title"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":"v","typeName":"TextView"}
{"color":-11899692,"id":"161","nextBlock":-1,"opCode":"setText","parameters":["@162","Can\u0027t get the IP address and network speed."],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"162","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["content"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":"v","typeName":"TextView"}
{"color":-1988310,"id":"128","nextBlock":-1,"opCode":"if","parameters":["@129"],"spec":"if %b then","subStack1":131,"subStack2":-1,"type":"c","typeName":""}
{"color":-10701022,"id":"129","nextBlock":-1,"opCode":"stringEquals","parameters":["@130","dark"],"spec":"%s equals %s","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-13851166,"id":"130","nextBlock":-1,"opCode":"fileGetData","parameters":["theme","theme"],"spec":"%m.file getData key %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-10701022,"id":"131","nextBlock":132,"opCode":"addSourceDirectly","parameters":["LayoutInflater Inflater \u003d getLayoutInflater();\n\nView InfView \u003d getLayoutInflater().inflate(R.layout.design_3, null);\n\nLinearLayout border \u003d (LinearLayout) InfView.findViewById(R.id.border);\n\nLinearLayout line_content \u003d (LinearLayout) InfView.findViewById(R.id.line_content);\n\nLinearLayout line \u003d (LinearLayout) InfView.findViewById (R.id.line);\n\nTextView title \u003d (TextView) InfView.findViewById(R.id.title);\n\nTextView content \u003d (TextView) InfView.findViewById(R.id.content);\n\nImageView image_logo \u003d (ImageView) InfView.findViewById(R.id.image_logo);\n\nToast ToastName \u003d Toast.makeText(getApplicationContext(),\"\",Toast.LENGTH_SHORT);\n\nToastName.setView(InfView);\n\nToastName.show();"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"132","nextBlock":133,"opCode":"addSourceDirectly","parameters":["image_logo.setImageResource(R.drawable.warn);\nborder.setElevation((float)2);\ntitle.setTextColor(Color.parseColor(\"#47D765\"));"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"133","nextBlock":135,"opCode":"setTextColor","parameters":["@134","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"134","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["title"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":"v","typeName":"TextView"}
{"color":-11899692,"id":"135","nextBlock":137,"opCode":"setTypeface","parameters":["@136","font_regular","bold"],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"136","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["title"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":"v","typeName":"TextView"}
{"color":-11899692,"id":"137","nextBlock":139,"opCode":"setTypeface","parameters":["@138","font_light","normal"],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"138","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["content"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":"v","typeName":"TextView"}
{"color":-7711273,"id":"139","nextBlock":141,"opCode":"definedFunc","parameters":["@140","#000000","0","#00000000","8"],"spec":"RoundAndBorder %m.view.view backColor %s.color1 border %d.border borderColor %s.color2 round %d.round","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"140","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["border"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":"v","typeName":"TextView"}
{"color":-7711273,"id":"141","nextBlock":143,"opCode":"definedFunc","parameters":["@142","#FFC122","0","#00000000","360"],"spec":"RoundAndBorder %m.view.view backColor %s.color1 border %d.border borderColor %s.color2 round %d.round","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"142","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["line"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":"v","typeName":"TextView"}
{"color":-11899692,"id":"143","nextBlock":145,"opCode":"setText","parameters":["@144","No internet connection!"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"144","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["title"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":"v","typeName":"TextView"}
{"color":-11899692,"id":"145","nextBlock":-1,"opCode":"setText","parameters":["@146","Can\u0027t get the IP address and network speed."],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"146","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["content"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":"v","typeName":"TextView"}

@DeviceinfoActivity.java_generalDeviceInfo_moreBlock
{"color":-11899692,"id":"136","nextBlock":10,"opCode":"setText","parameters":["title","General info"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"10","nextBlock":146,"opCode":"addSourceDirectly","parameters":["release \u003d Build.VERSION.RELEASE;\nmanufacturer \u003d Build.MANUFACTURER;\nmodel \u003d Build.MODEL;\nkernel \u003d System.getProperty(\"os.version\");\napi \u003d android.os.Build.VERSION.SDK;\nbootloader \u003d android.os.Build.BOOTLOADER;\nfingerprint \u003d Build.FINGERPRINT;\narch \u003d System.getProperty(\"os.arch\");"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"146","nextBlock":11,"opCode":"addSourceDirectly","parameters":["CPUABI \u003d Build.CPU_ABI;"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-1988310,"id":"11","nextBlock":21,"opCode":"if","parameters":["@12"],"spec":"if %b then","subStack1":18,"subStack2":-1,"type":"c","typeName":""}
{"color":-10701022,"id":"12","nextBlock":-1,"opCode":"||","parameters":["@13","@15"],"spec":"%b or %b","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-10701022,"id":"13","nextBlock":-1,"opCode":"stringEquals","parameters":["@14","21"],"spec":"%s equals %s","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-1147626,"id":"14","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"api","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-10701022,"id":"15","nextBlock":-1,"opCode":"stringEquals","parameters":["@16","22"],"spec":"%s equals %s","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-1147626,"id":"16","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"api","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-11899692,"id":"18","nextBlock":-1,"opCode":"setText","parameters":["show_android_version","@140"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"140","nextBlock":-1,"opCode":"stringJoin","parameters":["@19"," Lollipop"],"spec":"join %s and %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-10701022,"id":"19","nextBlock":-1,"opCode":"stringJoin","parameters":["Android ","@20"],"spec":"join %s and %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-1147626,"id":"20","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"release","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-1988310,"id":"21","nextBlock":28,"opCode":"if","parameters":["@22"],"spec":"if %b then","subStack1":25,"subStack2":-1,"type":"c","typeName":""}
{"color":-10701022,"id":"22","nextBlock":-1,"opCode":"stringEquals","parameters":["@23","23"],"spec":"%s equals %s","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-1147626,"id":"23","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"api","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-11899692,"id":"25","nextBlock":-1,"opCode":"setText","parameters":["show_android_version","@141"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"141","nextBlock":-1,"opCode":"stringJoin","parameters":["@26"," Marshmallow"],"spec":"join %s and %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-10701022,"id":"26","nextBlock":-1,"opCode":"stringJoin","parameters":["Android ","@27"],"spec":"join %s and %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-1147626,"id":"27","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"release","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-1988310,"id":"28","nextBlock":38,"opCode":"if","parameters":["@29"],"spec":"if %b then","subStack1":35,"subStack2":-1,"type":"c","typeName":""}
{"color":-10701022,"id":"29","nextBlock":-1,"opCode":"||","parameters":["@30","@32"],"spec":"%b or %b","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-10701022,"id":"30","nextBlock":-1,"opCode":"stringEquals","parameters":["@31","24"],"spec":"%s equals %s","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-1147626,"id":"31","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"api","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-10701022,"id":"32","nextBlock":-1,"opCode":"stringEquals","parameters":["@33","25"],"spec":"%s equals %s","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-1147626,"id":"33","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"api","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-11899692,"id":"35","nextBlock":-1,"opCode":"setText","parameters":["show_android_version","@142"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"142","nextBlock":-1,"opCode":"stringJoin","parameters":["@36"," Nougat"],"spec":"join %s and %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-10701022,"id":"36","nextBlock":-1,"opCode":"stringJoin","parameters":["Android ","@37"],"spec":"join %s and %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-1147626,"id":"37","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"release","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-1988310,"id":"38","nextBlock":48,"opCode":"if","parameters":["@39"],"spec":"if %b then","subStack1":45,"subStack2":-1,"type":"c","typeName":""}
{"color":-10701022,"id":"39","nextBlock":-1,"opCode":"||","parameters":["@40","@42"],"spec":"%b or %b","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-10701022,"id":"40","nextBlock":-1,"opCode":"stringEquals","parameters":["@41","26"],"spec":"%s equals %s","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-1147626,"id":"41","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"api","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-10701022,"id":"42","nextBlock":-1,"opCode":"stringEquals","parameters":["@43","27"],"spec":"%s equals %s","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-1147626,"id":"43","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"api","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-11899692,"id":"45","nextBlock":-1,"opCode":"setText","parameters":["show_android_version","@143"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"143","nextBlock":-1,"opCode":"stringJoin","parameters":["@46"," Oreo"],"spec":"join %s and %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-10701022,"id":"46","nextBlock":-1,"opCode":"stringJoin","parameters":["Android ","@47"],"spec":"join %s and %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-1147626,"id":"47","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"release","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-1988310,"id":"48","nextBlock":55,"opCode":"if","parameters":["@49"],"spec":"if %b then","subStack1":52,"subStack2":-1,"type":"c","typeName":""}
{"color":-10701022,"id":"49","nextBlock":-1,"opCode":"stringEquals","parameters":["@50","28"],"spec":"%s equals %s","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-1147626,"id":"50","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"api","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-11899692,"id":"52","nextBlock":-1,"opCode":"setText","parameters":["show_android_version","@144"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"144","nextBlock":-1,"opCode":"stringJoin","parameters":["@53"," Pie"],"spec":"join %s and %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-10701022,"id":"53","nextBlock":-1,"opCode":"stringJoin","parameters":["Android ","@54"],"spec":"join %s and %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-1147626,"id":"54","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"release","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-1988310,"id":"55","nextBlock":62,"opCode":"if","parameters":["@56"],"spec":"if %b then","subStack1":59,"subStack2":-1,"type":"c","typeName":""}
{"color":-10701022,"id":"56","nextBlock":-1,"opCode":"stringEquals","parameters":["@57","29"],"spec":"%s equals %s","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-1147626,"id":"57","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"api","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-11899692,"id":"59","nextBlock":-1,"opCode":"setText","parameters":["show_android_version","@145"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"145","nextBlock":-1,"opCode":"stringJoin","parameters":["@60"," Q"],"spec":"join %s and %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-10701022,"id":"60","nextBlock":-1,"opCode":"stringJoin","parameters":["Android ","@61"],"spec":"join %s and %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-1147626,"id":"61","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"release","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-1988310,"id":"62","nextBlock":69,"opCode":"if","parameters":["@63"],"spec":"if %b then","subStack1":73,"subStack2":-1,"type":"c","typeName":""}
{"color":-10701022,"id":"63","nextBlock":-1,"opCode":"stringEquals","parameters":["@64","30"],"spec":"%s equals %s","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-1147626,"id":"64","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"api","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-11899692,"id":"73","nextBlock":-1,"opCode":"setText","parameters":["show_android_version","@74"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"74","nextBlock":-1,"opCode":"stringJoin","parameters":["Android ","@75"],"spec":"join %s and %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-1147626,"id":"75","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"release","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-1988310,"id":"69","nextBlock":76,"opCode":"if","parameters":["@70"],"spec":"if %b then","subStack1":66,"subStack2":-1,"type":"c","typeName":""}
{"color":-10701022,"id":"70","nextBlock":-1,"opCode":"stringEquals","parameters":["@71","31"],"spec":"%s equals %s","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-1147626,"id":"71","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"api","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-11899692,"id":"66","nextBlock":-1,"opCode":"setText","parameters":["show_android_version","@67"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"67","nextBlock":-1,"opCode":"stringJoin","parameters":["Android ","@68"],"spec":"join %s and %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-1147626,"id":"68","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"release","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-10701022,"id":"76","nextBlock":77,"opCode":"addSourceDirectly","parameters":["try {"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"77","nextBlock":79,"opCode":"setText","parameters":["manufacturer_model","@92"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"92","nextBlock":-1,"opCode":"stringJoin","parameters":["@78","@93"],"spec":"join %s and %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-1147626,"id":"78","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"manufacturer","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-10701022,"id":"93","nextBlock":-1,"opCode":"stringJoin","parameters":["/","@80"],"spec":"join %s and %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-1147626,"id":"80","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"model","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-11899692,"id":"79","nextBlock":81,"opCode":"setText","parameters":["show_fp_arch","@97"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"97","nextBlock":-1,"opCode":"stringJoin","parameters":["@101","@99"],"spec":"join %s and %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-1147626,"id":"101","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"fingerprint","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-10701022,"id":"99","nextBlock":-1,"opCode":"stringJoin","parameters":["/","@102"],"spec":"join %s and %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-1147626,"id":"102","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"arch","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-11899692,"id":"81","nextBlock":83,"opCode":"setText","parameters":["device_bootloader_k","@94"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"94","nextBlock":-1,"opCode":"stringJoin","parameters":["@86","@96"],"spec":"join %s and %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-1147626,"id":"86","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"bootloader","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-10701022,"id":"96","nextBlock":-1,"opCode":"stringJoin","parameters":["/","@82"],"spec":"join %s and %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-1147626,"id":"82","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"kernel","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-11899692,"id":"83","nextBlock":147,"opCode":"setText","parameters":["show_device_sdk","@84"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-1147626,"id":"84","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"api","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-11899692,"id":"147","nextBlock":91,"opCode":"setText","parameters":["cpu_abi_show","@148"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-1147626,"id":"148","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"CPUABI","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-10701022,"id":"91","nextBlock":107,"opCode":"addSourceDirectly","parameters":["if (FileUtil.isExistFile(\"/sbin/su\") || FileUtil.isExistFile(\"/system/bin/su\") || FileUtil.isExistFile(\"/system/xbin/su\") ||\nFileUtil.isExistFile(\"/system/sbin/su\") ||\nFileUtil.isExistFile(\"/vendor/bin/su\")) {\n\nshow_root_status.setText(\"Rooted\");\n} else {\nshow_root_status.setText(\"Not rooted\");\n}\n} catch (Exception e) {\n}"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-1147626,"id":"107","nextBlock":109,"opCode":"setVarBoolean","parameters":["start","@108"],"spec":"set %m.varBool to %b","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"108","nextBlock":-1,"opCode":"true","parameters":[],"spec":"true","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-7711273,"id":"109","nextBlock":110,"opCode":"definedFunc","parameters":[],"spec":"ping_start","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"110","nextBlock":137,"opCode":"requestnetworkStartRequestNetwork","parameters":["ping_test","GET","https://www.google.com/","ping_test"],"spec":"%m.requestnetwork start network request to method %m.method to url %s with tag %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"137","nextBlock":149,"opCode":"requestnetworkStartRequestNetwork","parameters":["get_ip","GET","https://extreme-ip-lookup.com/json/",""],"spec":"%m.requestnetwork start network request to method %m.method to url %s with tag %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"149","nextBlock":116,"opCode":"addSourceDirectly","parameters":["DisplayMetrics dm \u003d new DisplayMetrics();\r\n getWindowManager().getDefaultDisplay().getMetrics(dm);\r\n\r\nint width \u003d dm.widthPixels;\r\n\r\nint height \u003d dm.heightPixels;\r\n\r\nscreen_size_show.setText(String.valueOf(height).concat(\" x \".concat(String.valueOf(width))));"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"116","nextBlock":117,"opCode":"definedFunc","parameters":["cat /proc/cpuinfo"],"spec":"execute_shell %s.command","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"117","nextBlock":129,"opCode":"setText","parameters":["show_cpu_info","@118"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-1147626,"id":"118","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"out","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-11899692,"id":"129","nextBlock":130,"opCode":"setVisible","parameters":["battery_info_main","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"130","nextBlock":131,"opCode":"setVisible","parameters":["cpu_ram_info_main","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"131","nextBlock":138,"opCode":"setVisible","parameters":["settings_main","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"138","nextBlock":133,"opCode":"setVisible","parameters":["network_info_main","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"133","nextBlock":134,"opCode":"setVisible","parameters":["dock2","INVISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"134","nextBlock":139,"opCode":"setVisible","parameters":["dock3","INVISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"139","nextBlock":135,"opCode":"setVisible","parameters":["dock4","INVISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"135","nextBlock":185,"opCode":"setVisible","parameters":["dock5","INVISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"185","nextBlock":-1,"opCode":"timerEvery","parameters":["timer","","1000"],"spec":"%m.timer after %d ms for every %d ms","subStack1":157,"subStack2":-1,"type":"c","typeName":""}
{"color":-13851166,"id":"157","nextBlock":150,"opCode":"calendarGetNow","parameters":["date_time"],"spec":"%m.calendar getNow","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"150","nextBlock":158,"opCode":"setText","parameters":["date_time_show","@154"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"154","nextBlock":-1,"opCode":"stringJoin","parameters":["@151","@155"],"spec":"join %s and %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-13851166,"id":"151","nextBlock":-1,"opCode":"calendarFormat","parameters":["date_time","HH:mm:ss"],"spec":"%m.calendar Format %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-10701022,"id":"155","nextBlock":-1,"opCode":"stringJoin","parameters":["\n","@153"],"spec":"join %s and %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-13851166,"id":"153","nextBlock":-1,"opCode":"calendarFormat","parameters":["date_time","dd/MM/yyyy"],"spec":"%m.calendar Format %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-7711273,"id":"158","nextBlock":159,"opCode":"definedFunc","parameters":["cat /proc/meminfo"],"spec":"execute_shell %s.command","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"159","nextBlock":160,"opCode":"addSourceDirectly","parameters":["// Getting the Total Memory\n\nString[] s \u003d out.split(\"\\n\")[0].split(\" \");\nmem_max \u003d Integer.parseInt(s[s.length - 2]);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"160","nextBlock":161,"opCode":"addSourceDirectly","parameters":["// Getting the current memory\n\nString[] l \u003d out.split(\"\\n\")[2].split(\" \");\nmem_usage \u003d mem_max - Integer.parseInt(l[l.length - 2]);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"161","nextBlock":163,"opCode":"seekBarSetMax","parameters":["ram_usage","@162"],"spec":"%m.seekbar setMax %d","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-1147626,"id":"162","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"mem_max","subStack1":-1,"subStack2":-1,"type":"d","typeName":""}
{"color":-11899692,"id":"163","nextBlock":165,"opCode":"seekBarSetProgress","parameters":["ram_usage","@164"],"spec":"%m.seekbar setProgress %d","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-1147626,"id":"164","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"mem_usage","subStack1":-1,"subStack2":-1,"type":"d","typeName":""}
{"color":-11899692,"id":"165","nextBlock":119,"opCode":"setText","parameters":["show_ram_usage","@166"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"166","nextBlock":-1,"opCode":"stringJoin","parameters":["Memory usage: ","@167"],"spec":"join %s and %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-10701022,"id":"167","nextBlock":-1,"opCode":"stringJoin","parameters":["@168","@171"],"spec":"join %s and %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-10701022,"id":"168","nextBlock":-1,"opCode":"toString","parameters":["@169"],"spec":"toString %d without decimal","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-10701022,"id":"169","nextBlock":-1,"opCode":"/","parameters":["@170","1024"],"spec":"%d / %d","subStack1":-1,"subStack2":-1,"type":"d","typeName":""}
{"color":-1147626,"id":"170","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"mem_usage","subStack1":-1,"subStack2":-1,"type":"d","typeName":""}
{"color":-10701022,"id":"171","nextBlock":-1,"opCode":"stringJoin","parameters":["@172","@177"],"spec":"join %s and %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-10701022,"id":"172","nextBlock":-1,"opCode":"stringJoin","parameters":[" MB / ","@173"],"spec":"join %s and %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-10701022,"id":"173","nextBlock":-1,"opCode":"stringJoin","parameters":["@174"," MB"],"spec":"join %s and %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-10701022,"id":"174","nextBlock":-1,"opCode":"toString","parameters":["@175"],"spec":"toString %d without decimal","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-10701022,"id":"175","nextBlock":-1,"opCode":"/","parameters":["@176","1024"],"spec":"%d / %d","subStack1":-1,"subStack2":-1,"type":"d","typeName":""}
{"color":-1147626,"id":"176","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"mem_max","subStack1":-1,"subStack2":-1,"type":"d","typeName":""}
{"color":-10701022,"id":"177","nextBlock":-1,"opCode":"stringJoin","parameters":[" (","@178"],"spec":"join %s and %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-10701022,"id":"178","nextBlock":-1,"opCode":"stringJoin","parameters":["@179","%)"],"spec":"join %s and %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-10701022,"id":"179","nextBlock":-1,"opCode":"toString","parameters":["@180"],"spec":"toString %d without decimal","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-10701022,"id":"180","nextBlock":-1,"opCode":"*","parameters":["@181","100"],"spec":"%d * %d","subStack1":-1,"subStack2":-1,"type":"d","typeName":""}
{"color":-10701022,"id":"181","nextBlock":-1,"opCode":"/","parameters":["@182","@183"],"spec":"%d / %d","subStack1":-1,"subStack2":-1,"type":"d","typeName":""}
{"color":-1147626,"id":"182","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"mem_usage","subStack1":-1,"subStack2":-1,"type":"d","typeName":""}
{"color":-1147626,"id":"183","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"mem_max","subStack1":-1,"subStack2":-1,"type":"d","typeName":""}
{"color":-7711273,"id":"119","nextBlock":120,"opCode":"definedFunc","parameters":["cat /proc/meminfo"],"spec":"execute_shell %s.command","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"120","nextBlock":112,"opCode":"setText","parameters":["show_ram_info","@121"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-1147626,"id":"121","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"out","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-11899692,"id":"112","nextBlock":103,"opCode":"seekBarSetProgress","parameters":["battery_progress","@113"],"spec":"%m.seekbar setProgress %d","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"113","nextBlock":-1,"opCode":"toNumber","parameters":["@114"],"spec":"toNumber %s","subStack1":-1,"subStack2":-1,"type":"d","typeName":""}
{"color":-10701022,"id":"114","nextBlock":-1,"opCode":"stringReplace","parameters":["@115","%"," "],"spec":"%s replace all %s with %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-11899692,"id":"115","nextBlock":-1,"opCode":"getText","parameters":["show_b_level"],"spec":"%m.textview getText","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-7711273,"id":"103","nextBlock":104,"opCode":"definedFunc","parameters":[],"spec":"Level","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"104","nextBlock":105,"opCode":"definedFunc","parameters":[],"spec":"Health","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"105","nextBlock":106,"opCode":"definedFunc","parameters":[],"spec":"Status","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"106","nextBlock":-1,"opCode":"definedFunc","parameters":[],"spec":"Temperature","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_onCreate_initializeLogic
{"color":-10701022,"id":"389","nextBlock":10,"opCode":"addSourceDirectly","parameters":["try{\ngetSupportActionBar().hide();\n} catch (Exception e){}"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"10","nextBlock":404,"opCode":"definedFunc","parameters":[],"spec":"generalDeviceInfo","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"404","nextBlock":408,"opCode":"addSourceDirectly","parameters":["String details \u003d \"VERSION.RELEASE : \"+Build.VERSION.RELEASE\n\n+\"\\nVERSION.INCREMENTAL : \"+Build.VERSION.INCREMENTAL\n\n+\"\\nVERSION.SDK.NUMBER : \"+Build.VERSION.SDK_INT\n\n+\"\\nBOARD : \"+Build.BOARD\n\n+\"\\nBOOTLOADER : \"+Build.BOOTLOADER\n\n+\"\\nBRAND : \"+Build.BRAND\n\n+\"\\nCPU_ABI : \"+Build.CPU_ABI\n\n+\"\\nCPU_ABI2 : \"+Build.CPU_ABI2\n\n+\"\\nDISPLAY : \"+Build.DISPLAY\n\n+\"\\nFINGERPRINT : \"+Build.FINGERPRINT\n\n+\"\\nHARDWARE : \"+Build.HARDWARE\n\n+\"\\nHOST : \"+Build.HOST\n\n+\"\\nID : \"+Build.ID\n\n+\"\\nMANUFACTURER : \"+Build.MANUFACTURER\n\n+\"\\nMODEL : \"+Build.MODEL\n\n+\"\\nPRODUCT : \"+Build.PRODUCT\n\n+\"\\nSERIAL : \"+Build.SERIAL\n\n+\"\\nTAGS : \"+Build.TAGS\n\n+\"\\nTIME : \"+Build.TIME\n\n+\"\\nTYPE : \"+Build.TYPE\n\n+\"\\nUNKNOWN : \"+Build.UNKNOWN\n\n+\"\\nUSER : \"+Build.USER;\n\n"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-1988310,"id":"408","nextBlock":409,"opCode":"if","parameters":["@81"],"spec":"if %b then","subStack1":412,"subStack2":-1,"type":"c","typeName":""}
{"color":-10701022,"id":"81","nextBlock":-1,"opCode":"stringEquals","parameters":["@82","dark"],"spec":"%s equals %s","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-13851166,"id":"82","nextBlock":-1,"opCode":"fileGetData","parameters":["theme","theme"],"spec":"%m.file getData key %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-11899692,"id":"412","nextBlock":568,"opCode":"setChecked","parameters":["switch_theme","@413"],"spec":"%m.checkbox setChecked %b","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"413","nextBlock":-1,"opCode":"true","parameters":[],"spec":"true","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-7711273,"id":"568","nextBlock":-1,"opCode":"definedFunc","parameters":[],"spec":"DarkUI","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-1988310,"id":"409","nextBlock":405,"opCode":"if","parameters":["@410"],"spec":"if %b then","subStack1":488,"subStack2":-1,"type":"c","typeName":""}
{"color":-10701022,"id":"410","nextBlock":-1,"opCode":"stringEquals","parameters":["@411","light"],"spec":"%s equals %s","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-13851166,"id":"411","nextBlock":-1,"opCode":"fileGetData","parameters":["theme","theme"],"spec":"%m.file getData key %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-11899692,"id":"488","nextBlock":569,"opCode":"setChecked","parameters":["switch_theme","@489"],"spec":"%m.checkbox setChecked %b","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"489","nextBlock":-1,"opCode":"false","parameters":[],"spec":"false","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-7711273,"id":"569","nextBlock":-1,"opCode":"definedFunc","parameters":[],"spec":"LightUI","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"405","nextBlock":242,"opCode":"setText","parameters":["show_fdi","@406"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-1147626,"id":"406","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"details","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-7711273,"id":"242","nextBlock":572,"opCode":"definedFunc","parameters":["bottom_nav_bar","20"],"spec":"Elevation %m.view.view set %d.number","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"572","nextBlock":573,"opCode":"dialogSetTitle","parameters":["change_log","App changelog"],"spec":"%m.dialog set title %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"573","nextBlock":574,"opCode":"dialogSetMessage","parameters":["change_log","• Version 1.0.release\nIntial release.\n• Version 1.2.2.3.release\nGrammar error fixed.\nAdded Refresh button.\nAdded Changelog.\nFixed the back button error (when you press back button in version 1.0.release then you are back to the splash screen.)\nPing changed to Internet speed.\nAdded press back button two times to exit.\nApp now use SharedPreferences for Dark theme so it can save your theme setting.\n• Version 1.2.2.4.release\nTitlebar error fixed (changed height from 56dp to wrap_content)\n• Version 1.4.4.4.release\nNetwork speed and IP address moved to Network info and added internet service provider info.\nAdded Elevation to the bottom navigation bar in app and added animation when clicked to the items.\nAdded Android version codename (Thanks Abhinandan for suggest this!)\n• Version 1.5.6.4.release\nAdded Drawer (please click on the  the title text to open the drawer.)\nRedesigned About page.\nAdded Full device info.\n• Version 1.5.7.5.release\nChanged IP API provider from ipify.org to extreme-ip-lookup.com\nAdded GPS location based on IP (not exactly at all)\nTo copy the location please hold on the text to copy.\n• Version 1.5.7.6.release\nFixed the GPS coordinates error.\n• Version 1.5.9.5.release\nAdded get country (based on the IP address) to the network info.\nRedesigned the About page (again).\n• Version 1.5.9.7.release\nFixed the error when the app crashed unexpectedly (GSON error).\nFixed the UI design error.\nFixed the splash screen error.\n• Version 1.5.10.7.release\nAdded CPU ABI info.\n• Version 1.5.11.7.release\nAdded device speaker cleaner tool.\n• Version 1.5.11.8.release\nFixed bug in dark theme.\n• Version 1.6.13.10.release\nAdded display test.\nAdded date and time.\nFixed the speaker cleaner.\nFixed some grammar error.\nAdded tools screen and added some animations to them.\n• Version 1.6.14.12.release\nAdded instructions to open the drawer.\nLogo changed.\nUI improvements.\nToast theme will change with the theme.\n• Version 1.6.20.15.release\nLogo changed (again)\nFixed some error with toast theme changing.\nLags reduced.\nRedesigned About page (again)\nToast style changed (Thanks to AnugDev\u0027s project!)\nLittle UI improvements.\nRoot checking method improved a little bit (Thanks to Ryenyuku and Yahir_Aguilera!)\nThe Date and Time changed a little bit.\nThe info will refresh every seconds so the refresh button will be removed (Thanks Sasuke for suggest this!)"],"spec":"%m.dialog set message %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"574","nextBlock":577,"opCode":"dialogNeutralButton","parameters":["change_log","Okay!"],"spec":"%m.dialog Neutral Button %s Clicked","subStack1":575,"subStack2":-1,"type":"c","typeName":""}
{"color":-13851166,"id":"575","nextBlock":576,"opCode":"fileSetData","parameters":["changelog","showdialog","false"],"spec":"%m.file setData key %s value %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"576","nextBlock":-1,"opCode":"dialogShow","parameters":["instructions"],"spec":"%m.dialog show","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"577","nextBlock":578,"opCode":"dialogSetTitle","parameters":["instructions","How to open the drawer?"],"spec":"%m.dialog set title %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"578","nextBlock":579,"opCode":"dialogSetMessage","parameters":["instructions","To open the drawer click on the title text on the top left corner to open.\nIn this drawer you can find some tools here."],"spec":"%m.dialog set message %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"579","nextBlock":237,"opCode":"dialogNeutralButton","parameters":["instructions","Okay!"],"spec":"%m.dialog Neutral Button %s Clicked","subStack1":580,"subStack2":-1,"type":"c","typeName":""}
{"color":-13851166,"id":"580","nextBlock":-1,"opCode":"fileSetData","parameters":["changelog","showdialog","false"],"spec":"%m.file setData key %s value %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-1988310,"id":"237","nextBlock":11,"opCode":"if","parameters":["@238"],"spec":"if %b then","subStack1":241,"subStack2":-1,"type":"c","typeName":""}
{"color":-10701022,"id":"238","nextBlock":-1,"opCode":"not","parameters":["@239"],"spec":"not %b","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-10701022,"id":"239","nextBlock":-1,"opCode":"stringEquals","parameters":["@240","false"],"spec":"%s equals %s","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-13851166,"id":"240","nextBlock":-1,"opCode":"fileGetData","parameters":["changelog","showdialog"],"spec":"%m.file getData key %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-13851166,"id":"241","nextBlock":-1,"opCode":"dialogShow","parameters":["change_log"],"spec":"%m.dialog show","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"11","nextBlock":12,"opCode":"definedFunc","parameters":[],"spec":"GUIFont","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"12","nextBlock":13,"opCode":"definedFunc","parameters":["dock1","#FF1100","#FF1100"],"spec":"Round %m.view.view Color1 %s.color1 Color2 %s.color2","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"13","nextBlock":14,"opCode":"definedFunc","parameters":["dock2","#FF1100","#FF1100"],"spec":"Round %m.view.view Color1 %s.color1 Color2 %s.color2","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"14","nextBlock":15,"opCode":"definedFunc","parameters":["dock3","#FF1100","#FF1100"],"spec":"Round %m.view.view Color1 %s.color1 Color2 %s.color2","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"15","nextBlock":243,"opCode":"definedFunc","parameters":["dock4","#FF1100","#FF1100"],"spec":"Round %m.view.view Color1 %s.color1 Color2 %s.color2","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"243","nextBlock":570,"opCode":"definedFunc","parameters":["dock5","#FF1100","#FF1100"],"spec":"Round %m.view.view Color1 %s.color1 Color2 %s.color2","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"570","nextBlock":-1,"opCode":"definedFunc","parameters":[],"spec":"drawerFont","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_aboutt_onClick
{"color":-1988310,"id":"10","nextBlock":-1,"opCode":"ifElse","parameters":["@11"],"spec":"if %b then","subStack1":13,"subStack2":16,"type":"e","typeName":""}
{"color":-10701022,"id":"11","nextBlock":-1,"opCode":"stringEquals","parameters":["@12","dark"],"spec":"%s equals %s","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-13851166,"id":"12","nextBlock":-1,"opCode":"fileGetData","parameters":["theme","theme"],"spec":"%m.file getData key %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-13851166,"id":"13","nextBlock":14,"opCode":"intentPutExtra","parameters":["go_to","theme","dark"],"spec":"%m.intent putExtra key %s value %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"14","nextBlock":15,"opCode":"intentSetScreen","parameters":["go_to","AboutActivity"],"spec":"%m.intent setScreen %m.activity","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"15","nextBlock":-1,"opCode":"startActivity","parameters":["go_to"],"spec":"StartActivity %m.intent","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"16","nextBlock":17,"opCode":"intentPutExtra","parameters":["go_to","theme","light"],"spec":"%m.intent putExtra key %s value %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"17","nextBlock":18,"opCode":"intentSetScreen","parameters":["go_to","AboutActivity"],"spec":"%m.intent setScreen %m.activity","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"18","nextBlock":-1,"opCode":"startActivity","parameters":["go_to"],"spec":"StartActivity %m.intent","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_LightUI_moreBlock
{"color":-7711273,"id":"10","nextBlock":11,"opCode":"definedFunc","parameters":[],"spec":"drawer_light","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"11","nextBlock":12,"opCode":"definedFunc","parameters":["#FFFFFF","#FFFFFF"],"spec":"status_bar_color %s.colour1 bottom_navigation_bar %s.colour2","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"12","nextBlock":13,"opCode":"definedFunc","parameters":[],"spec":"LightStatusBar","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"13","nextBlock":14,"opCode":"setTextColor","parameters":["title","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"14","nextBlock":15,"opCode":"setTextColor","parameters":["title_android_ver","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"15","nextBlock":16,"opCode":"setTextColor","parameters":["show_android_version","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"16","nextBlock":17,"opCode":"setTextColor","parameters":["title_device_sdk","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"17","nextBlock":18,"opCode":"setTextColor","parameters":["show_device_sdk","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"18","nextBlock":19,"opCode":"setTextColor","parameters":["title_android_ver","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"19","nextBlock":20,"opCode":"setTextColor","parameters":["title_dev_manu_model","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"20","nextBlock":21,"opCode":"setTextColor","parameters":["manufacturer_model","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"21","nextBlock":22,"opCode":"setTextColor","parameters":["dbk_title","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"22","nextBlock":23,"opCode":"setTextColor","parameters":["device_bootloader_k","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"23","nextBlock":24,"opCode":"setTextColor","parameters":["title_device_fp_arch","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"24","nextBlock":25,"opCode":"setTextColor","parameters":["show_fp_arch","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"25","nextBlock":26,"opCode":"setTextColor","parameters":["title_root_status","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"26","nextBlock":27,"opCode":"setTextColor","parameters":["show_root_status","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"27","nextBlock":28,"opCode":"setTextColor","parameters":["show_ip_address","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"28","nextBlock":29,"opCode":"setTextColor","parameters":["title_ip_address","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"29","nextBlock":30,"opCode":"setTextColor","parameters":["title_ping","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"30","nextBlock":31,"opCode":"setTextColor","parameters":["ping_view","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"31","nextBlock":32,"opCode":"setTextColor","parameters":["title_b_level","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"32","nextBlock":33,"opCode":"setTextColor","parameters":["show_b_level","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"33","nextBlock":34,"opCode":"setTextColor","parameters":["title_b_health","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"34","nextBlock":35,"opCode":"setTextColor","parameters":["battery_health","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"35","nextBlock":36,"opCode":"setTextColor","parameters":["title_b_temp","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"36","nextBlock":37,"opCode":"setTextColor","parameters":["battery_temperature","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"37","nextBlock":38,"opCode":"setTextColor","parameters":["title_b_status","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"38","nextBlock":39,"opCode":"setTextColor","parameters":["statust","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"39","nextBlock":40,"opCode":"setTextColor","parameters":["title_cpu_info","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"40","nextBlock":41,"opCode":"setTextColor","parameters":["show_cpu_info","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"41","nextBlock":42,"opCode":"setTextColor","parameters":["title_ram_usage","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"42","nextBlock":43,"opCode":"setTextColor","parameters":["show_ram_usage","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"43","nextBlock":44,"opCode":"setTextColor","parameters":["title_ram_info","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"44","nextBlock":45,"opCode":"setTextColor","parameters":["show_ram_info","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"45","nextBlock":46,"opCode":"setTextColor","parameters":["fdi_title","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"46","nextBlock":47,"opCode":"setTextColor","parameters":["show_fdi","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"47","nextBlock":48,"opCode":"setTextColor","parameters":["isp_title","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"48","nextBlock":49,"opCode":"setTextColor","parameters":["show_isp_name","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"49","nextBlock":50,"opCode":"setTextColor","parameters":["country_title","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"50","nextBlock":87,"opCode":"setTextColor","parameters":["country_show","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"87","nextBlock":85,"opCode":"setTextColor","parameters":["title_cpu_abi","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"85","nextBlock":86,"opCode":"setTextColor","parameters":["cpu_abi_show","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"86","nextBlock":51,"opCode":"setImage","parameters":["icon_cpu_abi","ic_memory_black"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"51","nextBlock":52,"opCode":"setTextColor","parameters":["switch_theme","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"52","nextBlock":53,"opCode":"setBgColor","parameters":["linear_title","0xFFFFFFFF"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"53","nextBlock":54,"opCode":"setBgColor","parameters":["scroll_main","0xFFFFFFFF"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"54","nextBlock":55,"opCode":"setBgColor","parameters":["bottom_nav_bar","0xFFFFFFFF"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"55","nextBlock":56,"opCode":"setBgColor","parameters":["main_linear","0xFFFFFFFF"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"56","nextBlock":57,"opCode":"setBgColor","parameters":["general_info_main","0xFFFFFFFF"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"57","nextBlock":58,"opCode":"setBgColor","parameters":["battery_info_main","0xFFFFFFFF"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"58","nextBlock":59,"opCode":"setBgColor","parameters":["cpu_ram_info_main","0xFFFFFFFF"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"59","nextBlock":91,"opCode":"setBgColor","parameters":["settings_main","0xFFFFFFFF"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"91","nextBlock":92,"opCode":"setTextColor","parameters":["date_time_title","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"92","nextBlock":93,"opCode":"setTextColor","parameters":["date_time_show","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"93","nextBlock":88,"opCode":"setImage","parameters":["dt_icon","ic_clock_black"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"88","nextBlock":89,"opCode":"setTextColor","parameters":["title_screen_size","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"89","nextBlock":90,"opCode":"setTextColor","parameters":["screen_size_show","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"90","nextBlock":60,"opCode":"setImage","parameters":["ic_screen_size","ic_perm_deviceinfo_black"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"60","nextBlock":61,"opCode":"setImage","parameters":["android_version_icon","ic_adb_black"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"61","nextBlock":62,"opCode":"setImage","parameters":["device_sdk_icon","ic_perm_deviceinfo_black"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"62","nextBlock":63,"opCode":"setImage","parameters":["manufacturer_icon","ic_local_convenience_store_black"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"63","nextBlock":64,"opCode":"setImage","parameters":["bug_icon","ic_bug_report_black"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"64","nextBlock":65,"opCode":"setImage","parameters":["fa_icon","ic_developer_mode_black"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"65","nextBlock":66,"opCode":"setImage","parameters":["root_icon","ic_dns_black"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"66","nextBlock":67,"opCode":"setImage","parameters":["icon_ip","ic_language_black"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"67","nextBlock":68,"opCode":"setImage","parameters":["icon_ping","ic_wifi_black"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"68","nextBlock":69,"opCode":"setImage","parameters":["b_icon","ic_battery_full_black"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"69","nextBlock":70,"opCode":"setImage","parameters":["b_icon_health","ic_battery_unknown_black"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"70","nextBlock":71,"opCode":"setImage","parameters":["b_icon_temp","ic_battery_full_black"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"71","nextBlock":72,"opCode":"setImage","parameters":["b_icon_status","ic_battery_full_black"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"72","nextBlock":73,"opCode":"setImage","parameters":["icon_cpu","ic_memory_black"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"73","nextBlock":74,"opCode":"setImage","parameters":["icon_ram","ic_ram_black"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"74","nextBlock":75,"opCode":"setImage","parameters":["ram_usage_icon","ic_ram_black"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"75","nextBlock":76,"opCode":"setImage","parameters":["fulldvinfo_icon","ic_perm_deviceinfo_black"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"76","nextBlock":77,"opCode":"setImage","parameters":["country_icon","ic_public_black"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"77","nextBlock":78,"opCode":"setImage","parameters":["icon_nightmode","ic_brightness_3_black"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"78","nextBlock":79,"opCode":"setImage","parameters":["isp_icon","ic_public_black"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"79","nextBlock":80,"opCode":"setImage","parameters":["","ic_refresh_black"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"80","nextBlock":81,"opCode":"setImage","parameters":["i_1","ic_perm_deviceinfo_black"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"81","nextBlock":82,"opCode":"setImage","parameters":["i_2","ic_battery_full_black"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"82","nextBlock":83,"opCode":"setImage","parameters":["i_3","ic_ram_black"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"83","nextBlock":84,"opCode":"setImage","parameters":["i_4","ic_public_black"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"84","nextBlock":-1,"opCode":"setImage","parameters":["i_5","ic_settings_black"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_DarkUI_moreBlock
{"color":-10701022,"id":"10","nextBlock":11,"opCode":"addSourceDirectly","parameters":["Window w \u003dDeviceinfoActivity.this.getWindow();\r\n\tw.clearFlags(WindowManager.LayoutParams.FLAG_TRANSLUCENT_STATUS);\r\n\tw.addFlags(WindowManager.LayoutParams.FLAG_DRAWS_SYSTEM_BAR_BACKGROUNDS);\n\nw.setStatusBarColor(0xFF000000);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"11","nextBlock":12,"opCode":"definedFunc","parameters":[],"spec":"drawer_dark","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"12","nextBlock":13,"opCode":"definedFunc","parameters":["#000000","#000000"],"spec":"status_bar_color %s.colour1 bottom_navigation_bar %s.colour2","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"13","nextBlock":14,"opCode":"setTextColor","parameters":["title","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"14","nextBlock":15,"opCode":"setTextColor","parameters":["title_android_ver","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"15","nextBlock":16,"opCode":"setTextColor","parameters":["show_android_version","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"16","nextBlock":17,"opCode":"setTextColor","parameters":["title_device_sdk","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"17","nextBlock":18,"opCode":"setTextColor","parameters":["show_device_sdk","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"18","nextBlock":19,"opCode":"setTextColor","parameters":["title_android_ver","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"19","nextBlock":20,"opCode":"setTextColor","parameters":["title_dev_manu_model","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"20","nextBlock":21,"opCode":"setTextColor","parameters":["manufacturer_model","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"21","nextBlock":22,"opCode":"setTextColor","parameters":["dbk_title","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"22","nextBlock":23,"opCode":"setTextColor","parameters":["device_bootloader_k","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"23","nextBlock":24,"opCode":"setTextColor","parameters":["title_device_fp_arch","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"24","nextBlock":25,"opCode":"setTextColor","parameters":["show_fp_arch","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"25","nextBlock":26,"opCode":"setTextColor","parameters":["title_root_status","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"26","nextBlock":27,"opCode":"setTextColor","parameters":["show_root_status","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"27","nextBlock":28,"opCode":"setTextColor","parameters":["show_ip_address","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"28","nextBlock":29,"opCode":"setTextColor","parameters":["title_ip_address","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"29","nextBlock":30,"opCode":"setTextColor","parameters":["title_ping","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"30","nextBlock":31,"opCode":"setTextColor","parameters":["ping_view","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"31","nextBlock":32,"opCode":"setTextColor","parameters":["title_b_level","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"32","nextBlock":33,"opCode":"setTextColor","parameters":["show_b_level","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"33","nextBlock":34,"opCode":"setTextColor","parameters":["title_b_health","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"34","nextBlock":35,"opCode":"setTextColor","parameters":["battery_health","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"35","nextBlock":36,"opCode":"setTextColor","parameters":["title_b_temp","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"36","nextBlock":37,"opCode":"setTextColor","parameters":["battery_temperature","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"37","nextBlock":38,"opCode":"setTextColor","parameters":["title_b_status","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"38","nextBlock":39,"opCode":"setTextColor","parameters":["statust","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"39","nextBlock":40,"opCode":"setTextColor","parameters":["title_cpu_info","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"40","nextBlock":41,"opCode":"setTextColor","parameters":["show_cpu_info","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"41","nextBlock":42,"opCode":"setTextColor","parameters":["title_ram_info","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"42","nextBlock":43,"opCode":"setTextColor","parameters":["show_ram_info","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"43","nextBlock":44,"opCode":"setTextColor","parameters":["title_ram_usage","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"44","nextBlock":45,"opCode":"setTextColor","parameters":["show_ram_usage","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"45","nextBlock":46,"opCode":"setTextColor","parameters":["fdi_title","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"46","nextBlock":47,"opCode":"setTextColor","parameters":["show_fdi","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"47","nextBlock":48,"opCode":"setTextColor","parameters":["isp_title","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"48","nextBlock":49,"opCode":"setTextColor","parameters":["show_isp_name","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"49","nextBlock":50,"opCode":"setTextColor","parameters":["country_title","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"50","nextBlock":85,"opCode":"setTextColor","parameters":["country_show","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"85","nextBlock":86,"opCode":"setTextColor","parameters":["title_cpu_abi","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"86","nextBlock":87,"opCode":"setTextColor","parameters":["cpu_abi_show","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"87","nextBlock":51,"opCode":"setImage","parameters":["icon_cpu_abi","ic_memory_white"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"51","nextBlock":52,"opCode":"setTextColor","parameters":["switch_theme","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"52","nextBlock":53,"opCode":"setBgColor","parameters":["linear_title","0xFF000000"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"53","nextBlock":54,"opCode":"setBgColor","parameters":["scroll_main","0xFF000000"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"54","nextBlock":55,"opCode":"setBgColor","parameters":["bottom_nav_bar","0xFF000000"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"55","nextBlock":56,"opCode":"setBgColor","parameters":["main_linear","0xFF000000"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"56","nextBlock":57,"opCode":"setBgColor","parameters":["general_info_main","0xFF000000"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"57","nextBlock":58,"opCode":"setBgColor","parameters":["battery_info_main","0xFF000000"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"58","nextBlock":59,"opCode":"setBgColor","parameters":["cpu_ram_info_main","0xFF000000"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"59","nextBlock":88,"opCode":"setBgColor","parameters":["settings_main","0xFF000000"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"88","nextBlock":89,"opCode":"setTextColor","parameters":["title_screen_size","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"89","nextBlock":90,"opCode":"setTextColor","parameters":["screen_size_show","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"90","nextBlock":91,"opCode":"setImage","parameters":["ic_screen_size","ic_perm_deviceinfo_white"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"91","nextBlock":92,"opCode":"setTextColor","parameters":["date_time_title","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"92","nextBlock":93,"opCode":"setTextColor","parameters":["date_time_show","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"93","nextBlock":60,"opCode":"setImage","parameters":["dt_icon","ic_clock_white"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"60","nextBlock":61,"opCode":"setImage","parameters":["android_version_icon","ic_adb_white"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"61","nextBlock":62,"opCode":"setImage","parameters":["device_sdk_icon","ic_perm_deviceinfo_white"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"62","nextBlock":63,"opCode":"setImage","parameters":["manufacturer_icon","ic_local_convenience_store_white"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"63","nextBlock":64,"opCode":"setImage","parameters":["bug_icon","ic_bug_report_white"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"64","nextBlock":65,"opCode":"setImage","parameters":["fa_icon","ic_developer_mode_white"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"65","nextBlock":66,"opCode":"setImage","parameters":["root_icon","ic_dns_white"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"66","nextBlock":67,"opCode":"setImage","parameters":["icon_ip","ic_language_white"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"67","nextBlock":68,"opCode":"setImage","parameters":["icon_ping","ic_wifi_white"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"68","nextBlock":69,"opCode":"setImage","parameters":["b_icon","ic_battery_full_white"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"69","nextBlock":70,"opCode":"setImage","parameters":["b_icon_health","ic_battery_unknown_white"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"70","nextBlock":71,"opCode":"setImage","parameters":["b_icon_temp","ic_battery_full_white"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"71","nextBlock":72,"opCode":"setImage","parameters":["b_icon_status","ic_battery_full_white"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"72","nextBlock":73,"opCode":"setImage","parameters":["icon_cpu","ic_memory_white"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"73","nextBlock":74,"opCode":"setImage","parameters":["icon_ram","ic_ram_white"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"74","nextBlock":75,"opCode":"setImage","parameters":["ram_usage_icon","ic_ram_white"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"75","nextBlock":76,"opCode":"setImage","parameters":["fulldvinfo_icon","ic_perm_deviceinfo_white"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"76","nextBlock":77,"opCode":"setImage","parameters":["country_icon","ic_public_white"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"77","nextBlock":78,"opCode":"setImage","parameters":["icon_nightmode","ic_brightness_3_white"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"78","nextBlock":79,"opCode":"setImage","parameters":["isp_icon","ic_public_white"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"79","nextBlock":80,"opCode":"setImage","parameters":["","ic_refresh_white"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"80","nextBlock":81,"opCode":"setImage","parameters":["i_1","ic_perm_deviceinfo_white"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"81","nextBlock":82,"opCode":"setImage","parameters":["i_2","ic_battery_full_white"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"82","nextBlock":83,"opCode":"setImage","parameters":["i_3","ic_ram_white"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"83","nextBlock":84,"opCode":"setImage","parameters":["i_4","ic_public_white"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"84","nextBlock":-1,"opCode":"setImage","parameters":["i_5","ic_settings_white"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_cpui_onClick
{"color":-7711273,"id":"10","nextBlock":-1,"opCode":"definedFunc","parameters":["show_cpu_info"],"spec":"copyToClipboard %m.textview.textview","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_RoundAndBorder_moreBlock
{"color":-10701022,"id":"10","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["android.graphics.drawable.GradientDrawable gd \u003d new android.graphics.drawable.GradientDrawable();\ngd.setColor(Color.parseColor(_color1));\ngd.setCornerRadius((int) _round);\ngd.setStroke((int) _border, Color.parseColor(_color2));\n_view.setBackground(gd);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_ram_progress_moreBlock
{"color":-7711273,"id":"11","nextBlock":12,"opCode":"definedFunc","parameters":["cat /proc/meminfo"],"spec":"execute_shell %s.command","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"12","nextBlock":13,"opCode":"addSourceDirectly","parameters":["// Getting the Total Memory\n\nString[] s \u003d out.split(\"\\n\")[0].split(\" \");\nmem_max \u003d Integer.parseInt(s[s.length - 2]);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"13","nextBlock":14,"opCode":"addSourceDirectly","parameters":["// Getting the current memory\n\nString[] l \u003d out.split(\"\\n\")[2].split(\" \");\nmem_usage \u003d mem_max - Integer.parseInt(l[l.length - 2]);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"14","nextBlock":16,"opCode":"seekBarSetMax","parameters":["ram_usage","@15"],"spec":"%m.seekbar setMax %d","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-1147626,"id":"15","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"mem_max","subStack1":-1,"subStack2":-1,"type":"d","typeName":""}
{"color":-11899692,"id":"16","nextBlock":18,"opCode":"seekBarSetProgress","parameters":["ram_usage","@17"],"spec":"%m.seekbar setProgress %d","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-1147626,"id":"17","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"mem_usage","subStack1":-1,"subStack2":-1,"type":"d","typeName":""}
{"color":-11899692,"id":"18","nextBlock":-1,"opCode":"setText","parameters":["show_ram_usage","@19"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"19","nextBlock":-1,"opCode":"stringJoin","parameters":["Memory Usage: ","@20"],"spec":"join %s and %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-10701022,"id":"20","nextBlock":-1,"opCode":"stringJoin","parameters":["@21","@24"],"spec":"join %s and %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-10701022,"id":"21","nextBlock":-1,"opCode":"toString","parameters":["@22"],"spec":"toString %d without decimal","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-10701022,"id":"22","nextBlock":-1,"opCode":"/","parameters":["@23","1024"],"spec":"%d / %d","subStack1":-1,"subStack2":-1,"type":"d","typeName":""}
{"color":-1147626,"id":"23","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"mem_usage","subStack1":-1,"subStack2":-1,"type":"d","typeName":""}
{"color":-10701022,"id":"24","nextBlock":-1,"opCode":"stringJoin","parameters":["@25","@30"],"spec":"join %s and %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-10701022,"id":"25","nextBlock":-1,"opCode":"stringJoin","parameters":[" MB / ","@26"],"spec":"join %s and %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-10701022,"id":"26","nextBlock":-1,"opCode":"stringJoin","parameters":["@27"," MB"],"spec":"join %s and %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-10701022,"id":"27","nextBlock":-1,"opCode":"toString","parameters":["@28"],"spec":"toString %d without decimal","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-10701022,"id":"28","nextBlock":-1,"opCode":"/","parameters":["@29","1024"],"spec":"%d / %d","subStack1":-1,"subStack2":-1,"type":"d","typeName":""}
{"color":-1147626,"id":"29","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"mem_max","subStack1":-1,"subStack2":-1,"type":"d","typeName":""}
{"color":-10701022,"id":"30","nextBlock":-1,"opCode":"stringJoin","parameters":[" (","@31"],"spec":"join %s and %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-10701022,"id":"31","nextBlock":-1,"opCode":"stringJoin","parameters":["@32","%)"],"spec":"join %s and %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-10701022,"id":"32","nextBlock":-1,"opCode":"toString","parameters":["@33"],"spec":"toString %d without decimal","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-10701022,"id":"33","nextBlock":-1,"opCode":"*","parameters":["@34","100"],"spec":"%d * %d","subStack1":-1,"subStack2":-1,"type":"d","typeName":""}
{"color":-10701022,"id":"34","nextBlock":-1,"opCode":"/","parameters":["@35","@36"],"spec":"%d / %d","subStack1":-1,"subStack2":-1,"type":"d","typeName":""}
{"color":-1147626,"id":"35","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"mem_usage","subStack1":-1,"subStack2":-1,"type":"d","typeName":""}
{"color":-1147626,"id":"36","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"mem_max","subStack1":-1,"subStack2":-1,"type":"d","typeName":""}

@DeviceinfoActivity.java_Health_moreBlock
{"color":-10701022,"id":"35","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["BroadcastReceiver healthbb \u003d new BroadcastReceiver() {\n\t@Override\n\tpublic void onReceive(Context context, Intent intent) {\n\t\tint healthbc \u003d intent.getIntExtra(BatteryManager.EXTRA_HEALTH,0);\n\t\tif (healthbc \u003d\u003d BatteryManager.BATTERY_HEALTH_DEAD) {\r\n\t\t\tbattery_health.setText(\"Dead\");\r\n\t\t}\r\n\t\tif (healthbc \u003d\u003d BatteryManager.BATTERY_HEALTH_GOOD) {\r\n\t\t\tbattery_health.setText(\"Good\");\r\n\t\t}\r\n\t\tif (healthbc \u003d\u003d BatteryManager.BATTERY_HEALTH_OVERHEAT) {\r\n\t\t\tbattery_health.setText(\"Overheat\");\r\n\t\t}\r\n\t\tif (healthbc \u003d\u003d BatteryManager.BATTERY_HEALTH_OVER_VOLTAGE) {\r\n\t\t\tbattery_health.setText(\"Over voltage\");\r\n\t\t}\r\n\t\tif (healthbc \u003d\u003d BatteryManager.BATTERY_HEALTH_UNKNOWN) {\r\n\t\t\tbattery_health.setText(\"Unknown battery health\");\r\n\t\t}\r\n\t\tif (healthbc \u003d\u003d BatteryManager.BATTERY_HEALTH_UNSPECIFIED_FAILURE) {\r\n\t\t\tbattery_health.setText(\"Unspecified failure battery\");\r\n\t\t}\n\t}\n};\r\nIntentFilter healthba \u003d new IntentFilter(Intent.ACTION_BATTERY_CHANGED);\ngetApplicationContext().registerReceiver(healthbb,healthba);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_linear_home_onClick
{"color":-11899692,"id":"21","nextBlock":10,"opCode":"closeDrawer","parameters":[],"spec":"closeDrawer","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"10","nextBlock":11,"opCode":"setText","parameters":["title","General info"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"11","nextBlock":12,"opCode":"setVisible","parameters":["general_info_main","VISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"12","nextBlock":13,"opCode":"setVisible","parameters":["battery_info_main","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"13","nextBlock":14,"opCode":"setVisible","parameters":["cpu_ram_info_main","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"14","nextBlock":15,"opCode":"setVisible","parameters":["settings_main","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"15","nextBlock":16,"opCode":"setVisible","parameters":["network_info_main","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"16","nextBlock":17,"opCode":"setVisible","parameters":["dock1","VISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"17","nextBlock":18,"opCode":"setVisible","parameters":["dock2","INVISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"18","nextBlock":19,"opCode":"setVisible","parameters":["dock3","INVISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"19","nextBlock":20,"opCode":"setVisible","parameters":["dock4","INVISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"20","nextBlock":-1,"opCode":"setVisible","parameters":["dock5","INVISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_status_bar_color_moreBlock
{"color":-10701022,"id":"10","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["if (Build.VERSION.SDK_INT \u003e Build.VERSION_CODES.LOLLIPOP) { \n   Window w \u003d this.getWindow(); w.clearFlags(WindowManager.LayoutParams.FLAG_TRANSLUCENT_STATUS); w.addFlags(WindowManager.LayoutParams.FLAG_DRAWS_SYSTEM_BAR_BACKGROUNDS);\n   w.setStatusBarColor(Color.parseColor(_colour1)); w.setNavigationBarColor(Color.parseColor(_colour2));\n}"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_df_onClick
{"color":-7711273,"id":"10","nextBlock":-1,"opCode":"definedFunc","parameters":["show_fp_arch"],"spec":"copyToClipboard %m.textview.textview","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_bstatus_onClick
{"color":-7711273,"id":"10","nextBlock":-1,"opCode":"definedFunc","parameters":["statust"],"spec":"copyToClipboard %m.textview.textview","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_ping_start_moreBlock
{"color":-1988310,"id":"10","nextBlock":-1,"opCode":"if","parameters":["@11"],"spec":"if %b then","subStack1":12,"subStack2":-1,"type":"c","typeName":""}
{"color":-1147626,"id":"11","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"start","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-1147626,"id":"12","nextBlock":13,"opCode":"increaseInt","parameters":["ping"],"spec":"%m.varInt increase 1","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"13","nextBlock":-1,"opCode":"timerAfter","parameters":["ping_timer","1"],"spec":"%m.timer after %d ms","subStack1":14,"subStack2":-1,"type":"c","typeName":""}
{"color":-7711273,"id":"14","nextBlock":-1,"opCode":"definedFunc","parameters":[],"spec":"ping_start","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_Animator_moreBlock
{"color":-10701022,"id":"10","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["ObjectAnimator anim \u003d new ObjectAnimator();\nanim.setTarget(_view);\nanim.setPropertyName(_propertyName);\nanim.setFloatValues((float)_value);\nanim.setDuration((long)_duration);\nanim.start();"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_Level_moreBlock
{"color":-10701022,"id":"13","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["BatteryManager blevelq \u003d (BatteryManager)getSystemService(BATTERY_SERVICE);\nint blevelw \u003d blevelq.getIntProperty(BatteryManager.BATTERY_PROPERTY_CAPACITY);\nString blevele \u003d Integer.toString(blevelw);\r\nshow_b_level.setText(blevele.concat(\"%\"));"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_execute_shell_moreBlock
{"color":-10701022,"id":"10","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["StringBuilder output \u003d new StringBuilder();\ntry {\njava.lang.Process cmdProc \u003d Runtime.getRuntime().exec(_command);\n\n\njava.io.BufferedReader stdoutReader \u003d new java.io.BufferedReader(\n         new java.io.InputStreamReader(cmdProc.getInputStream()));\nString line;\nwhile ((line \u003d stdoutReader.readLine()) !\u003d null) {\n\n  output.append(line + \"\\n\");\n}\n\nthis.out \u003d output.toString();\n\n} catch (java.io.IOException e) {\nthis.out \u003d \"Error occurred\";\n}"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_title_onClick
{"color":-11899692,"id":"10","nextBlock":-1,"opCode":"openDrawer","parameters":[],"spec":"openDrawer","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_item_2_onClick
{"color":-11899692,"id":"20","nextBlock":21,"opCode":"setText","parameters":["title","Battery info"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"21","nextBlock":22,"opCode":"setVisible","parameters":["general_info_main","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"22","nextBlock":23,"opCode":"setVisible","parameters":["battery_info_main","VISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"23","nextBlock":24,"opCode":"setVisible","parameters":["cpu_ram_info_main","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"24","nextBlock":25,"opCode":"setVisible","parameters":["settings_main","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"25","nextBlock":26,"opCode":"setVisible","parameters":["network_info_main","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"26","nextBlock":27,"opCode":"setVisible","parameters":["dock1","INVISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"27","nextBlock":28,"opCode":"setVisible","parameters":["dock2","VISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"28","nextBlock":29,"opCode":"setVisible","parameters":["dock3","INVISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"29","nextBlock":30,"opCode":"setVisible","parameters":["dock4","INVISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"30","nextBlock":31,"opCode":"setVisible","parameters":["dock5","INVISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"31","nextBlock":-1,"opCode":"definedFunc","parameters":["i_2"],"spec":"clickAnimation %m.view.view","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_clickAnimation_moreBlock
{"color":-10701022,"id":"10","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["ScaleAnimation fade_in \u003d new ScaleAnimation(0.9f, 1f, 0.9f, 1f, Animation.RELATIVE_TO_SELF, 0.5f, Animation.RELATIVE_TO_SELF, 0.7f);\nfade_in.setDuration(300);\nfade_in.setFillAfter(true);\n_view.startAnimation(fade_in);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_ip_onClick
{"color":-7711273,"id":"10","nextBlock":-1,"opCode":"definedFunc","parameters":["show_ip_address"],"spec":"copyToClipboard %m.textview.textview","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_item_4_onClick
{"color":-11899692,"id":"10","nextBlock":11,"opCode":"setText","parameters":["title","Network info"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"11","nextBlock":12,"opCode":"setVisible","parameters":["general_info_main","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"12","nextBlock":13,"opCode":"setVisible","parameters":["battery_info_main","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"13","nextBlock":14,"opCode":"setVisible","parameters":["cpu_ram_info_main","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"14","nextBlock":15,"opCode":"setVisible","parameters":["settings_main","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"15","nextBlock":16,"opCode":"setVisible","parameters":["network_info_main","VISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"16","nextBlock":17,"opCode":"setVisible","parameters":["dock1","INVISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"17","nextBlock":18,"opCode":"setVisible","parameters":["dock2","INVISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"18","nextBlock":19,"opCode":"setVisible","parameters":["dock3","INVISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"19","nextBlock":20,"opCode":"setVisible","parameters":["dock4","VISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"20","nextBlock":21,"opCode":"setVisible","parameters":["dock5","INVISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"21","nextBlock":-1,"opCode":"definedFunc","parameters":["i_4"],"spec":"clickAnimation %m.view.view","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_onBackPressed_onBackPressed
{"color":-1988310,"id":"34","nextBlock":12,"opCode":"if","parameters":["@35"],"spec":"if %b then","subStack1":37,"subStack2":-1,"type":"c","typeName":""}
{"color":-11899692,"id":"35","nextBlock":-1,"opCode":"isDrawerOpen","parameters":[],"spec":"isDrawerOpen","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-1147626,"id":"37","nextBlock":36,"opCode":"setVarInt","parameters":["count","0"],"spec":"set %m.varInt to %d","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"36","nextBlock":-1,"opCode":"closeDrawer","parameters":[],"spec":"closeDrawer","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-1147626,"id":"12","nextBlock":16,"opCode":"increaseInt","parameters":["count"],"spec":"%m.varInt increase 1","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-1988310,"id":"16","nextBlock":13,"opCode":"if","parameters":["@18"],"spec":"if %b then","subStack1":68,"subStack2":-1,"type":"c","typeName":""}
{"color":-10701022,"id":"18","nextBlock":-1,"opCode":"\u003d","parameters":["@17","1"],"spec":"%d \u003d %d","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-1147626,"id":"17","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"count","subStack1":-1,"subStack2":-1,"type":"d","typeName":""}
{"color":-1988310,"id":"68","nextBlock":87,"opCode":"if","parameters":["@69"],"spec":"if %b then","subStack1":71,"subStack2":-1,"type":"c","typeName":""}
{"color":-10701022,"id":"69","nextBlock":-1,"opCode":"stringEquals","parameters":["@70","light"],"spec":"%s equals %s","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-13851166,"id":"70","nextBlock":-1,"opCode":"fileGetData","parameters":["theme","theme"],"spec":"%m.file getData key %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-10701022,"id":"71","nextBlock":72,"opCode":"addSourceDirectly","parameters":["LayoutInflater Inflater \u003d getLayoutInflater();\n\nView InfView \u003d getLayoutInflater().inflate(R.layout.design_3, null);\n\nLinearLayout border \u003d (LinearLayout) InfView.findViewById(R.id.border);\n\nLinearLayout line_content \u003d (LinearLayout) InfView.findViewById(R.id.line_content);\n\nLinearLayout line \u003d (LinearLayout) InfView.findViewById (R.id.line);\n\nTextView title \u003d (TextView) InfView.findViewById(R.id.title);\n\nTextView content \u003d (TextView) InfView.findViewById(R.id.content);\n\nImageView image_logo \u003d (ImageView) InfView.findViewById(R.id.image_logo);\n\nToast ToastName \u003d Toast.makeText(getApplicationContext(),\"\",Toast.LENGTH_SHORT);\n\nToastName.setView(InfView);\n\nToastName.show();"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"72","nextBlock":73,"opCode":"addSourceDirectly","parameters":["image_logo.setImageResource(R.drawable.warn);\nborder.setElevation((float)2);\ntitle.setTextColor(Color.parseColor(\"#47D765\"));"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"73","nextBlock":75,"opCode":"setTextColor","parameters":["@74","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"74","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["title"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":"v","typeName":"TextView"}
{"color":-11899692,"id":"75","nextBlock":77,"opCode":"setTypeface","parameters":["@76","font_light","bold"],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"76","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["title"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":"v","typeName":"TextView"}
{"color":-11899692,"id":"77","nextBlock":79,"opCode":"setTypeface","parameters":["@78","font_regular","normal"],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"78","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["content"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":"v","typeName":"TextView"}
{"color":-7711273,"id":"79","nextBlock":81,"opCode":"definedFunc","parameters":["@80","#ffffff","0","#00000000","8"],"spec":"RoundAndBorder %m.view.view backColor %s.color1 border %d.border borderColor %s.color2 round %d.round","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"80","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["border"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":"v","typeName":"TextView"}
{"color":-7711273,"id":"81","nextBlock":83,"opCode":"definedFunc","parameters":["@82","#FCC122","0","#00000000","360"],"spec":"RoundAndBorder %m.view.view backColor %s.color1 border %d.border borderColor %s.color2 round %d.round","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"82","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["line"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":"v","typeName":"TextView"}
{"color":-11899692,"id":"83","nextBlock":85,"opCode":"setText","parameters":["@84","Warning!"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"84","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["title"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":"v","typeName":"TextView"}
{"color":-11899692,"id":"85","nextBlock":-1,"opCode":"setText","parameters":["@86","Press again to exit!"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"86","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["content"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":"v","typeName":"TextView"}
{"color":-1988310,"id":"87","nextBlock":-1,"opCode":"if","parameters":["@88"],"spec":"if %b then","subStack1":90,"subStack2":-1,"type":"c","typeName":""}
{"color":-10701022,"id":"88","nextBlock":-1,"opCode":"stringEquals","parameters":["@89","dark"],"spec":"%s equals %s","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-13851166,"id":"89","nextBlock":-1,"opCode":"fileGetData","parameters":["theme","theme"],"spec":"%m.file getData key %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-10701022,"id":"90","nextBlock":91,"opCode":"addSourceDirectly","parameters":["LayoutInflater Inflater \u003d getLayoutInflater();\n\nView InfView \u003d getLayoutInflater().inflate(R.layout.design_3, null);\n\nLinearLayout border \u003d (LinearLayout) InfView.findViewById(R.id.border);\n\nLinearLayout line_content \u003d (LinearLayout) InfView.findViewById(R.id.line_content);\n\nLinearLayout line \u003d (LinearLayout) InfView.findViewById (R.id.line);\n\nTextView title \u003d (TextView) InfView.findViewById(R.id.title);\n\nTextView content \u003d (TextView) InfView.findViewById(R.id.content);\n\nImageView image_logo \u003d (ImageView) InfView.findViewById(R.id.image_logo);\n\nToast ToastName \u003d Toast.makeText(getApplicationContext(),\"\",Toast.LENGTH_SHORT);\n\nToastName.setView(InfView);\n\nToastName.show();"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"91","nextBlock":92,"opCode":"addSourceDirectly","parameters":["image_logo.setImageResource(R.drawable.warn);\nborder.setElevation((float)2);\ntitle.setTextColor(Color.parseColor(\"#47D765\"));"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"92","nextBlock":94,"opCode":"setTextColor","parameters":["@93","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"93","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["title"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":"v","typeName":"TextView"}
{"color":-11899692,"id":"94","nextBlock":96,"opCode":"setTypeface","parameters":["@95","font_regular","bold"],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"95","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["title"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":"v","typeName":"TextView"}
{"color":-11899692,"id":"96","nextBlock":98,"opCode":"setTypeface","parameters":["@97","font_light","normal"],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"97","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["content"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":"v","typeName":"TextView"}
{"color":-7711273,"id":"98","nextBlock":100,"opCode":"definedFunc","parameters":["@99","#000000","0","#00000000","8"],"spec":"RoundAndBorder %m.view.view backColor %s.color1 border %d.border borderColor %s.color2 round %d.round","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"99","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["border"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":"v","typeName":"TextView"}
{"color":-7711273,"id":"100","nextBlock":102,"opCode":"definedFunc","parameters":["@101","#FFC122","0","#00000000","360"],"spec":"RoundAndBorder %m.view.view backColor %s.color1 border %d.border borderColor %s.color2 round %d.round","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"101","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["line"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":"v","typeName":"TextView"}
{"color":-11899692,"id":"102","nextBlock":104,"opCode":"setText","parameters":["@103","Warning"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"103","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["title"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":"v","typeName":"TextView"}
{"color":-11899692,"id":"104","nextBlock":-1,"opCode":"setText","parameters":["@105","Press again to exit!"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"105","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["content"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":"v","typeName":"TextView"}
{"color":-1988310,"id":"13","nextBlock":-1,"opCode":"if","parameters":["@14"],"spec":"if %b then","subStack1":10,"subStack2":-1,"type":"c","typeName":""}
{"color":-10701022,"id":"14","nextBlock":-1,"opCode":"\u003d","parameters":["@15","2"],"spec":"%d \u003d %d","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-1147626,"id":"15","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"count","subStack1":-1,"subStack2":-1,"type":"d","typeName":""}
{"color":-10701022,"id":"10","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["finishAffinity();"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_dbk_onClick
{"color":-7711273,"id":"10","nextBlock":-1,"opCode":"definedFunc","parameters":["device_bootloader_k"],"spec":"copyToClipboard %m.textview.textview","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_background_onClick
{"color":-7711273,"id":"10","nextBlock":-1,"opCode":"definedFunc","parameters":["show_b_level"],"spec":"copyToClipboard %m.textview.textview","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_aver_onClick
{"color":-7711273,"id":"31","nextBlock":-1,"opCode":"definedFunc","parameters":["show_android_version"],"spec":"copyToClipboard %m.textview.textview","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_copyToClipboard_moreBlock
{"color":-1988310,"id":"31","nextBlock":32,"opCode":"if","parameters":["@29"],"spec":"if %b then","subStack1":10,"subStack2":-1,"type":"c","typeName":""}
{"color":-10701022,"id":"29","nextBlock":-1,"opCode":"stringEquals","parameters":["@30","light"],"spec":"%s equals %s","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-13851166,"id":"30","nextBlock":-1,"opCode":"fileGetData","parameters":["theme","theme"],"spec":"%m.file getData key %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-13851166,"id":"10","nextBlock":43,"opCode":"copyToClipboard","parameters":["@11"],"spec":"copyToClipboard %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"11","nextBlock":-1,"opCode":"getText","parameters":["@27"],"spec":"%m.textview getText","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-7711273,"id":"27","nextBlock":-1,"opCode":"getArg","parameters":[],"spec":"textview","subStack1":-1,"subStack2":-1,"type":"v","typeName":"TextView"}
{"color":-10701022,"id":"43","nextBlock":44,"opCode":"addSourceDirectly","parameters":["LayoutInflater Inflater \u003d getLayoutInflater();\n\nView InfView \u003d getLayoutInflater().inflate(R.layout.design_3, null);\n\nLinearLayout border \u003d (LinearLayout) InfView.findViewById(R.id.border);\n\nLinearLayout line_content \u003d (LinearLayout) InfView.findViewById(R.id.line_content);\n\nLinearLayout line \u003d (LinearLayout) InfView.findViewById (R.id.line);\n\nTextView title \u003d (TextView) InfView.findViewById(R.id.title);\n\nTextView content \u003d (TextView) InfView.findViewById(R.id.content);\n\nImageView image_logo \u003d (ImageView) InfView.findViewById(R.id.image_logo);\n\nToast ToastName \u003d Toast.makeText(getApplicationContext(),\"\",Toast.LENGTH_SHORT);\n\nToastName.setView(InfView);\n\nToastName.show();"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"44","nextBlock":72,"opCode":"addSourceDirectly","parameters":["image_logo.setImageResource(R.drawable.success);\nborder.setElevation((float)2);\ntitle.setTextColor(Color.parseColor(\"#47D765\"));"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"72","nextBlock":45,"opCode":"setTextColor","parameters":["@73","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"73","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["title"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":"v","typeName":"TextView"}
{"color":-11899692,"id":"45","nextBlock":47,"opCode":"setTypeface","parameters":["@46","font_light","bold"],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"46","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["title"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":"v","typeName":"TextView"}
{"color":-11899692,"id":"47","nextBlock":49,"opCode":"setTypeface","parameters":["@48","font_regular","normal"],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"48","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["content"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":"v","typeName":"TextView"}
{"color":-7711273,"id":"49","nextBlock":51,"opCode":"definedFunc","parameters":["@50","#ffffff","0","#00000000","8"],"spec":"RoundAndBorder %m.view.view backColor %s.color1 border %d.border borderColor %s.color2 round %d.round","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"50","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["border"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":"v","typeName":"TextView"}
{"color":-7711273,"id":"51","nextBlock":53,"opCode":"definedFunc","parameters":["@52","#47D765","0","#00000000","360"],"spec":"RoundAndBorder %m.view.view backColor %s.color1 border %d.border borderColor %s.color2 round %d.round","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"52","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["line"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":"v","typeName":"TextView"}
{"color":-11899692,"id":"53","nextBlock":55,"opCode":"setText","parameters":["@54","Success!"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"54","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["title"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":"v","typeName":"TextView"}
{"color":-11899692,"id":"55","nextBlock":-1,"opCode":"setText","parameters":["@56","Copied to clipboard!"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"56","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["content"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":"v","typeName":"TextView"}
{"color":-1988310,"id":"32","nextBlock":-1,"opCode":"if","parameters":["@33"],"spec":"if %b then","subStack1":40,"subStack2":-1,"type":"c","typeName":""}
{"color":-10701022,"id":"33","nextBlock":-1,"opCode":"stringEquals","parameters":["@34","dark"],"spec":"%s equals %s","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-13851166,"id":"34","nextBlock":-1,"opCode":"fileGetData","parameters":["theme","theme"],"spec":"%m.file getData key %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-13851166,"id":"40","nextBlock":57,"opCode":"copyToClipboard","parameters":["@41"],"spec":"copyToClipboard %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"41","nextBlock":-1,"opCode":"getText","parameters":["@42"],"spec":"%m.textview getText","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-7711273,"id":"42","nextBlock":-1,"opCode":"getArg","parameters":[],"spec":"textview","subStack1":-1,"subStack2":-1,"type":"v","typeName":"TextView"}
{"color":-10701022,"id":"57","nextBlock":58,"opCode":"addSourceDirectly","parameters":["LayoutInflater Inflater \u003d getLayoutInflater();\n\nView InfView \u003d getLayoutInflater().inflate(R.layout.design_3, null);\n\nLinearLayout border \u003d (LinearLayout) InfView.findViewById(R.id.border);\n\nLinearLayout line_content \u003d (LinearLayout) InfView.findViewById(R.id.line_content);\n\nLinearLayout line \u003d (LinearLayout) InfView.findViewById (R.id.line);\n\nTextView title \u003d (TextView) InfView.findViewById(R.id.title);\n\nTextView content \u003d (TextView) InfView.findViewById(R.id.content);\n\nImageView image_logo \u003d (ImageView) InfView.findViewById(R.id.image_logo);\n\nToast ToastName \u003d Toast.makeText(getApplicationContext(),\"\",Toast.LENGTH_SHORT);\n\nToastName.setView(InfView);\n\nToastName.show();"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"58","nextBlock":71,"opCode":"addSourceDirectly","parameters":["image_logo.setImageResource(R.drawable.success);\nborder.setElevation((float)2);\ntitle.setTextColor(Color.parseColor(\"#47D765\"));"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"71","nextBlock":59,"opCode":"setTextColor","parameters":["@74","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"74","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["title"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":"v","typeName":"TextView"}
{"color":-11899692,"id":"59","nextBlock":61,"opCode":"setTypeface","parameters":["@60","font_regular","bold"],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"60","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["title"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":"v","typeName":"TextView"}
{"color":-11899692,"id":"61","nextBlock":63,"opCode":"setTypeface","parameters":["@62","font_light","normal"],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"62","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["content"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":"v","typeName":"TextView"}
{"color":-7711273,"id":"63","nextBlock":65,"opCode":"definedFunc","parameters":["@64","#000000","0","#00000000","8"],"spec":"RoundAndBorder %m.view.view backColor %s.color1 border %d.border borderColor %s.color2 round %d.round","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"64","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["border"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":"v","typeName":"TextView"}
{"color":-7711273,"id":"65","nextBlock":67,"opCode":"definedFunc","parameters":["@66","#47D765","0","#00000000","360"],"spec":"RoundAndBorder %m.view.view backColor %s.color1 border %d.border borderColor %s.color2 round %d.round","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"66","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["line"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":"v","typeName":"TextView"}
{"color":-11899692,"id":"67","nextBlock":69,"opCode":"setText","parameters":["@68","Success!"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"68","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["title"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":"v","typeName":"TextView"}
{"color":-11899692,"id":"69","nextBlock":-1,"opCode":"setText","parameters":["@70","Copied to clipboard!"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"70","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["content"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":"v","typeName":"TextView"}

@DeviceinfoActivity.java_dsdk_onClick
{"color":-7711273,"id":"27","nextBlock":-1,"opCode":"definedFunc","parameters":["show_device_sdk"],"spec":"copyToClipboard %m.textview.textview","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_linear8_onClick
{"color":-7711273,"id":"10","nextBlock":-1,"opCode":"definedFunc","parameters":["country_show"],"spec":"copyToClipboard %m.textview.textview","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_get_ip_onResponse
{"color":-10701022,"id":"10","nextBlock":12,"opCode":"strToMap","parameters":["@11","get"],"spec":"Json %s to %m.varMap","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"11","nextBlock":-1,"opCode":"getArg","parameters":[],"spec":"response","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-11899692,"id":"12","nextBlock":14,"opCode":"setText","parameters":["show_ip_address","@13"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-1147626,"id":"13","nextBlock":-1,"opCode":"mapGet","parameters":["get","query"],"spec":"%m.varMap get key %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-11899692,"id":"14","nextBlock":34,"opCode":"setText","parameters":["show_isp_name","@26"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"26","nextBlock":-1,"opCode":"stringJoin","parameters":["@21","@31"],"spec":"join %s and %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-1147626,"id":"21","nextBlock":-1,"opCode":"mapGet","parameters":["get","isp"],"spec":"%m.varMap get key %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-10701022,"id":"31","nextBlock":-1,"opCode":"stringJoin","parameters":["\n","@28"],"spec":"join %s and %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-10701022,"id":"28","nextBlock":-1,"opCode":"stringJoin","parameters":["@27","@29"],"spec":"join %s and %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-1147626,"id":"27","nextBlock":-1,"opCode":"mapGet","parameters":["get","lat"],"spec":"%m.varMap get key %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-10701022,"id":"29","nextBlock":-1,"opCode":"stringJoin","parameters":[", ","@30"],"spec":"join %s and %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-1147626,"id":"30","nextBlock":-1,"opCode":"mapGet","parameters":["get","lon"],"spec":"%m.varMap get key %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-11899692,"id":"34","nextBlock":-1,"opCode":"setText","parameters":["country_show","@35"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-1147626,"id":"35","nextBlock":-1,"opCode":"mapGet","parameters":["get","country"],"spec":"%m.varMap get key %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}

@DeviceinfoActivity.java_drawer_dark_moreBlock
{"color":-10701022,"id":"10","nextBlock":11,"opCode":"addSourceDirectly","parameters":["_drawer_linear1.setBackgroundColor(Color.parseColor(\"#000000\"));"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"11","nextBlock":12,"opCode":"addSourceDirectly","parameters":["_drawer_title.setTextColor(0xFFFFFFFF);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"12","nextBlock":13,"opCode":"addSourceDirectly","parameters":["_drawer_subtitle.setTextColor(0xFFFFFFFF);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"13","nextBlock":14,"opCode":"addSourceDirectly","parameters":["_drawer_general.setTextColor(0xFFFFFFFF);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"14","nextBlock":15,"opCode":"addSourceDirectly","parameters":["_drawer_battery.setTextColor(0xFFFFFFFF);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"15","nextBlock":16,"opCode":"addSourceDirectly","parameters":["_drawer_cpuaram.setTextColor(0xFFFFFFFF);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"16","nextBlock":17,"opCode":"addSourceDirectly","parameters":["_drawer_network.setTextColor(0xFFFFFFFF);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"17","nextBlock":27,"opCode":"addSourceDirectly","parameters":["_drawer_sett.setTextColor(0xFFFFFFFF);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"27","nextBlock":24,"opCode":"addSourceDirectly","parameters":["_drawer_fix_device.setTextColor(0xFFFFFFFF);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"24","nextBlock":18,"opCode":"addSourceDirectly","parameters":["_drawer_about.setTextColor(0xFFFFFFFF);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"18","nextBlock":19,"opCode":"addSourceDirectly","parameters":["_drawer_i1.setImageResource(R.drawable.ic_perm_deviceinfo_white);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"19","nextBlock":20,"opCode":"addSourceDirectly","parameters":["_drawer_i2.setImageResource(R.drawable.ic_battery_full_white);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"20","nextBlock":21,"opCode":"addSourceDirectly","parameters":["_drawer_i3.setImageResource(R.drawable.ic_ram_white);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"21","nextBlock":22,"opCode":"addSourceDirectly","parameters":["_drawer_i4.setImageResource(R.drawable.ic_public_white);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"22","nextBlock":26,"opCode":"addSourceDirectly","parameters":["_drawer_i5.setImageResource(R.drawable.ic_settings_white);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"26","nextBlock":23,"opCode":"addSourceDirectly","parameters":["_drawer_fix_icon.setImageResource(R.drawable.ic_tune_white);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"23","nextBlock":25,"opCode":"addSourceDirectly","parameters":["_drawer_i6.setImageResource(R.drawable.ic_info_outline_white);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"25","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["_drawer_close.setImageResource(R.drawable.ic_clear_white);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_btemp_onClick
{"color":-7711273,"id":"10","nextBlock":-1,"opCode":"definedFunc","parameters":["battery_temperature"],"spec":"copyToClipboard %m.textview.textview","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_fix_onClick
{"color":-1988310,"id":"12","nextBlock":-1,"opCode":"ifElse","parameters":["@13"],"spec":"if %b then","subStack1":15,"subStack2":18,"type":"e","typeName":""}
{"color":-10701022,"id":"13","nextBlock":-1,"opCode":"stringEquals","parameters":["@14","dark"],"spec":"%s equals %s","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-13851166,"id":"14","nextBlock":-1,"opCode":"fileGetData","parameters":["theme","theme"],"spec":"%m.file getData key %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-13851166,"id":"15","nextBlock":16,"opCode":"intentPutExtra","parameters":["go_to","theme","dark"],"spec":"%m.intent putExtra key %s value %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"16","nextBlock":17,"opCode":"intentSetScreen","parameters":["go_to","ToolsActivity"],"spec":"%m.intent setScreen %m.activity","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"17","nextBlock":-1,"opCode":"startActivity","parameters":["go_to"],"spec":"StartActivity %m.intent","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"18","nextBlock":19,"opCode":"intentPutExtra","parameters":["go_to","theme","light"],"spec":"%m.intent putExtra key %s value %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"19","nextBlock":20,"opCode":"intentSetScreen","parameters":["go_to","ToolsActivity"],"spec":"%m.intent setScreen %m.activity","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"20","nextBlock":-1,"opCode":"startActivity","parameters":["go_to"],"spec":"StartActivity %m.intent","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_Temperature_moreBlock
{"color":-10701022,"id":"14","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["BroadcastReceiver btempz \u003d new BroadcastReceiver() {\n\t@Override\n\tpublic void onReceive(Context context, Intent intent) {\n\t\tfloat btempy \u003d (float)(intent.getIntExtra(BatteryManager.EXTRA_TEMPERATURE,0))/10;\n\t\tbattery_temperature.setText(btempy +\" \"+ (char) 0x00B0 +\"C\");\n\t}\n};\r\nIntentFilter btempx \u003d new IntentFilter(Intent.ACTION_BATTERY_CHANGED);\ngetApplicationContext().registerReceiver(btempz,btempx);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_drawer_light_moreBlock
{"color":-10701022,"id":"10","nextBlock":11,"opCode":"addSourceDirectly","parameters":["_drawer_linear1.setBackgroundColor(Color.parseColor(\"#FFFFFF\"));"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"11","nextBlock":12,"opCode":"addSourceDirectly","parameters":["_drawer_title.setTextColor(0xFF000000);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"12","nextBlock":13,"opCode":"addSourceDirectly","parameters":["_drawer_subtitle.setTextColor(0xFF000000);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"13","nextBlock":14,"opCode":"addSourceDirectly","parameters":["_drawer_general.setTextColor(0xFF000000);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"14","nextBlock":15,"opCode":"addSourceDirectly","parameters":["_drawer_battery.setTextColor(0xFF000000);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"15","nextBlock":16,"opCode":"addSourceDirectly","parameters":["_drawer_cpuaram.setTextColor(0xFF000000);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"16","nextBlock":17,"opCode":"addSourceDirectly","parameters":["_drawer_network.setTextColor(0xFF000000);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"17","nextBlock":26,"opCode":"addSourceDirectly","parameters":["_drawer_sett.setTextColor(0xFF000000);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"26","nextBlock":18,"opCode":"addSourceDirectly","parameters":["_drawer_fix_device.setTextColor(0xFF000000);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"18","nextBlock":19,"opCode":"addSourceDirectly","parameters":["_drawer_about.setTextColor(0xFF000000);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"19","nextBlock":20,"opCode":"addSourceDirectly","parameters":["_drawer_i1.setImageResource(R.drawable.ic_perm_deviceinfo_black);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"20","nextBlock":21,"opCode":"addSourceDirectly","parameters":["_drawer_i2.setImageResource(R.drawable.ic_battery_full_black);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"21","nextBlock":22,"opCode":"addSourceDirectly","parameters":["_drawer_i3.setImageResource(R.drawable.ic_ram_black);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"22","nextBlock":23,"opCode":"addSourceDirectly","parameters":["_drawer_i4.setImageResource(R.drawable.ic_public_black);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"23","nextBlock":27,"opCode":"addSourceDirectly","parameters":["_drawer_i5.setImageResource(R.drawable.ic_settings_black);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"27","nextBlock":24,"opCode":"addSourceDirectly","parameters":["_drawer_fix_icon.setImageResource(R.drawable.ic_tune_black);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"24","nextBlock":25,"opCode":"addSourceDirectly","parameters":["_drawer_i6.setImageResource(R.drawable.ic_info_outline_black);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"25","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["_drawer_close.setImageResource(R.drawable.ic_close_black);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DeviceinfoActivity.java_switch_theme_onCheckedChange
{"color":-1988310,"id":"10","nextBlock":-1,"opCode":"ifElse","parameters":["@11"],"spec":"if %b then","subStack1":164,"subStack2":165,"type":"e","typeName":""}
{"color":-7711273,"id":"11","nextBlock":-1,"opCode":"getArg","parameters":[],"spec":"isChecked","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-13851166,"id":"164","nextBlock":188,"opCode":"fileSetData","parameters":["theme","theme","dark"],"spec":"%m.file setData key %s value %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"188","nextBlock":-1,"opCode":"definedFunc","parameters":[],"spec":"DarkUI","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"165","nextBlock":189,"opCode":"fileSetData","parameters":["theme","theme","light"],"spec":"%m.file setData key %s value %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"189","nextBlock":-1,"opCode":"definedFunc","parameters":[],"spec":"LightUI","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@MainActivity.java_rippleRoundStroke_moreBlock
{"color":-10701022,"id":"16","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["android.graphics.drawable.GradientDrawable GG \u003d new android.graphics.drawable.GradientDrawable();\nGG.setColor(Color.parseColor(_focus));\nGG.setCornerRadius((float)_round);\nGG.setStroke((int) _stroke,\nColor.parseColor(\"#\" + _strokeclr.replace(\"#\", \"\")));\nandroid.graphics.drawable.RippleDrawable RE \u003d new android.graphics.drawable.RippleDrawable(new android.content.res.ColorStateList(new int[][]{new int[]{}}, new int[]{ Color.parseColor(_pressed)}), GG, null);\n_view.setBackground(RE);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@MainActivity.java_onCreate_initializeLogic
{"color":-7711273,"id":"28","nextBlock":-1,"opCode":"definedFunc","parameters":[],"spec":"logic","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@MainActivity.java_status_bar_color_moreBlock
{"color":-10701022,"id":"10","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["if (Build.VERSION.SDK_INT \u003e Build.VERSION_CODES.LOLLIPOP) { \n   Window w \u003d this.getWindow(); w.clearFlags(WindowManager.LayoutParams.FLAG_TRANSLUCENT_STATUS); w.addFlags(WindowManager.LayoutParams.FLAG_DRAWS_SYSTEM_BAR_BACKGROUNDS);\n   w.setStatusBarColor(Color.parseColor(_colour1)); w.setNavigationBarColor(Color.parseColor(_colour2));\n}"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@MainActivity.java_onResume_onResume
{"color":-7711273,"id":"23","nextBlock":10,"opCode":"definedFunc","parameters":["linear2","#FFFFFF","#FFFFFF","0","0","#FFFFFF"],"spec":"rippleRoundStroke %m.view.view onFocus %s.focus onPressed %s.pressed setRound %d.round setStroke %d.stroke setStrokeColor %s.strokeclr","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"10","nextBlock":11,"opCode":"objectanimatorSetTarget","parameters":["object","linear2"],"spec":"%m.objectanimator set target %m.view","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"11","nextBlock":12,"opCode":"objectanimatorSetProperty","parameters":["object","alpha"],"spec":"%m.objectanimator set property %m.animatorproperty","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"12","nextBlock":13,"opCode":"objectanimatorSetFromTo","parameters":["object","0","1"],"spec":"%m.objectanimator set values from %d to %d","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"13","nextBlock":14,"opCode":"objectanimatorSetDuration","parameters":["object","500"],"spec":"%m.objectanimator set duration %d","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"14","nextBlock":15,"opCode":"objectanimatorStart","parameters":["object"],"spec":"%m.objectanimator start","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"15","nextBlock":24,"opCode":"timerAfter","parameters":["timer","3000"],"spec":"%m.timer after %d ms","subStack1":16,"subStack2":-1,"type":"c","typeName":""}
{"color":-13851166,"id":"16","nextBlock":17,"opCode":"objectanimatorSetTarget","parameters":["object","linear2"],"spec":"%m.objectanimator set target %m.view","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"17","nextBlock":18,"opCode":"objectanimatorSetProperty","parameters":["object","scaleX"],"spec":"%m.objectanimator set property %m.animatorproperty","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"18","nextBlock":19,"opCode":"objectanimatorSetFromTo","parameters":["object","1","30"],"spec":"%m.objectanimator set values from %d to %d","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"19","nextBlock":20,"opCode":"objectanimatorSetDuration","parameters":["object","1200"],"spec":"%m.objectanimator set duration %d","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"20","nextBlock":21,"opCode":"objectanimatorStart","parameters":["object"],"spec":"%m.objectanimator start","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"21","nextBlock":22,"opCode":"definedFunc","parameters":[],"spec":"imageScale","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"22","nextBlock":-1,"opCode":"definedFunc","parameters":[],"spec":"lscaleY","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"24","nextBlock":25,"opCode":"definedFunc","parameters":["linear2","0"],"spec":"Elevation %m.view.view set %d.number","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"25","nextBlock":-1,"opCode":"definedFunc","parameters":["#FFFFFF","#FFFFFF"],"spec":"status_bar_color %s.colour1 bottom_navigation_bar %s.colour2","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@MainActivity.java_imageScale_moreBlock
{"color":-13851166,"id":"10","nextBlock":11,"opCode":"objectanimatorSetTarget","parameters":["d","app_logo"],"spec":"%m.objectanimator set target %m.view","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"11","nextBlock":12,"opCode":"objectanimatorSetProperty","parameters":["d","scaleX"],"spec":"%m.objectanimator set property %m.animatorproperty","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"12","nextBlock":13,"opCode":"objectanimatorSetFromTo","parameters":["d","1","0"],"spec":"%m.objectanimator set values from %d to %d","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"13","nextBlock":14,"opCode":"objectanimatorSetDuration","parameters":["d","300"],"spec":"%m.objectanimator set duration %d","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"14","nextBlock":21,"opCode":"objectanimatorStart","parameters":["d"],"spec":"%m.objectanimator start","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-1988310,"id":"21","nextBlock":-1,"opCode":"if","parameters":["@15"],"spec":"if %b then","subStack1":16,"subStack2":-1,"type":"c","typeName":""}
{"color":-10701022,"id":"15","nextBlock":-1,"opCode":"true","parameters":[],"spec":"true","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-13851166,"id":"16","nextBlock":17,"opCode":"objectanimatorSetTarget","parameters":["bd","app_logo"],"spec":"%m.objectanimator set target %m.view","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"17","nextBlock":18,"opCode":"objectanimatorSetProperty","parameters":["bd","scaleY"],"spec":"%m.objectanimator set property %m.animatorproperty","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"18","nextBlock":19,"opCode":"objectanimatorSetFromTo","parameters":["bd","1","0"],"spec":"%m.objectanimator set values from %d to %d","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"19","nextBlock":20,"opCode":"objectanimatorSetDuration","parameters":["bd","300"],"spec":"%m.objectanimator set duration %d","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"20","nextBlock":-1,"opCode":"objectanimatorStart","parameters":["bd"],"spec":"%m.objectanimator start","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@MainActivity.java_Elevation_moreBlock
{"color":-10701022,"id":"10","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["\n_view.setElevation((int)_number);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@MainActivity.java_lscaleY_moreBlock
{"color":-13851166,"id":"10","nextBlock":11,"opCode":"objectanimatorSetTarget","parameters":["c","linear2"],"spec":"%m.objectanimator set target %m.view","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"11","nextBlock":12,"opCode":"objectanimatorSetProperty","parameters":["c","scaleY"],"spec":"%m.objectanimator set property %m.animatorproperty","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"12","nextBlock":13,"opCode":"objectanimatorSetFromTo","parameters":["c","1","30"],"spec":"%m.objectanimator set values from %d to %d","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"13","nextBlock":14,"opCode":"objectanimatorSetDuration","parameters":["c","1200"],"spec":"%m.objectanimator set duration %d","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"14","nextBlock":15,"opCode":"objectanimatorStart","parameters":["c"],"spec":"%m.objectanimator start","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"15","nextBlock":-1,"opCode":"timerAfter","parameters":["timer","350"],"spec":"%m.timer after %d ms","subStack1":16,"subStack2":-1,"type":"c","typeName":""}
{"color":-13851166,"id":"16","nextBlock":17,"opCode":"intentSetScreen","parameters":["go_to","DeviceinfoActivity"],"spec":"%m.intent setScreen %m.activity","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"17","nextBlock":-1,"opCode":"startActivity","parameters":["go_to"],"spec":"StartActivity %m.intent","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@MainActivity.java_logic_moreBlock
{"color":-10701022,"id":"10","nextBlock":11,"opCode":"addSourceDirectly","parameters":["getWindow().getDecorView().setSystemUiVisibility(View.SYSTEM_UI_FLAG_LIGHT_STATUS_BAR);\r\ngetWindow().setStatusBarColor(0xFFFFFFFF);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"11","nextBlock":12,"opCode":"setTypeface","parameters":["","font_light",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"12","nextBlock":13,"opCode":"definedFunc","parameters":["linear2","#FFFFFF","#FFFFFF","0","0","#FFFFFF"],"spec":"rippleRoundStroke %m.view.view onFocus %s.focus onPressed %s.pressed setRound %d.round setStroke %d.stroke setStrokeColor %s.strokeclr","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"13","nextBlock":14,"opCode":"objectanimatorSetTarget","parameters":["object","linear2"],"spec":"%m.objectanimator set target %m.view","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"14","nextBlock":15,"opCode":"objectanimatorSetProperty","parameters":["object","alpha"],"spec":"%m.objectanimator set property %m.animatorproperty","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"15","nextBlock":16,"opCode":"objectanimatorSetFromTo","parameters":["object","0","1"],"spec":"%m.objectanimator set values from %d to %d","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"16","nextBlock":17,"opCode":"objectanimatorSetDuration","parameters":["object","500"],"spec":"%m.objectanimator set duration %d","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"17","nextBlock":18,"opCode":"objectanimatorStart","parameters":["object"],"spec":"%m.objectanimator start","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"18","nextBlock":26,"opCode":"timerAfter","parameters":["timer","3000"],"spec":"%m.timer after %d ms","subStack1":19,"subStack2":-1,"type":"c","typeName":""}
{"color":-13851166,"id":"19","nextBlock":20,"opCode":"objectanimatorSetTarget","parameters":["object","linear2"],"spec":"%m.objectanimator set target %m.view","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"20","nextBlock":21,"opCode":"objectanimatorSetProperty","parameters":["object","scaleX"],"spec":"%m.objectanimator set property %m.animatorproperty","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"21","nextBlock":22,"opCode":"objectanimatorSetFromTo","parameters":["object","1","30"],"spec":"%m.objectanimator set values from %d to %d","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"22","nextBlock":23,"opCode":"objectanimatorSetDuration","parameters":["object","1200"],"spec":"%m.objectanimator set duration %d","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"23","nextBlock":24,"opCode":"objectanimatorStart","parameters":["object"],"spec":"%m.objectanimator start","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"24","nextBlock":25,"opCode":"definedFunc","parameters":[],"spec":"imageScale","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"25","nextBlock":-1,"opCode":"definedFunc","parameters":[],"spec":"lscaleY","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"26","nextBlock":27,"opCode":"definedFunc","parameters":["linear2","0"],"spec":"Elevation %m.view.view set %d.number","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"27","nextBlock":-1,"opCode":"definedFunc","parameters":["#FFFFFF","#FFFFFF"],"spec":"status_bar_color %s.colour1 bottom_navigation_bar %s.colour2","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DisplayTestActivity.java_onCreate_initializeLogic
{"color":-7711273,"id":"83","nextBlock":84,"opCode":"definedFunc","parameters":["color_bars"],"spec":"Background setType %s.type","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"84","nextBlock":87,"opCode":"timerAfter","parameters":["time_switch","1500"],"spec":"%m.timer after %d ms","subStack1":85,"subStack2":-1,"type":"c","typeName":""}
{"color":-7711273,"id":"85","nextBlock":86,"opCode":"definedFunc","parameters":["transparent"],"spec":"Background setType %s.type","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"86","nextBlock":-1,"opCode":"definedFunc","parameters":["blackbars"],"spec":"Background setType %s.type","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"87","nextBlock":-1,"opCode":"timerEvery","parameters":["time_switch","1900","25"],"spec":"%m.timer after %d ms for every %d ms","subStack1":88,"subStack2":-1,"type":"c","typeName":""}
{"color":-7711273,"id":"88","nextBlock":89,"opCode":"definedFunc","parameters":["transparent"],"spec":"Background setType %s.type","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"89","nextBlock":-1,"opCode":"timerAfter","parameters":["time_switch","25"],"spec":"%m.timer after %d ms","subStack1":90,"subStack2":-1,"type":"c","typeName":""}
{"color":-7711273,"id":"90","nextBlock":91,"opCode":"definedFunc","parameters":["red"],"spec":"Background setType %s.type","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"91","nextBlock":-1,"opCode":"timerAfter","parameters":["time_switch","25"],"spec":"%m.timer after %d ms","subStack1":92,"subStack2":-1,"type":"c","typeName":""}
{"color":-7711273,"id":"92","nextBlock":93,"opCode":"definedFunc","parameters":["green"],"spec":"Background setType %s.type","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"93","nextBlock":-1,"opCode":"timerAfter","parameters":["time_switch","25"],"spec":"%m.timer after %d ms","subStack1":94,"subStack2":-1,"type":"c","typeName":""}
{"color":-7711273,"id":"94","nextBlock":-1,"opCode":"definedFunc","parameters":["blue"],"spec":"Background setType %s.type","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@DisplayTestActivity.java_Background_moreBlock
{"color":-1988310,"id":"10","nextBlock":13,"opCode":"if","parameters":["@12"],"spec":"if %b then","subStack1":24,"subStack2":-1,"type":"c","typeName":""}
{"color":-10701022,"id":"12","nextBlock":-1,"opCode":"stringEquals","parameters":["@11","color_bars"],"spec":"%s equals %s","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-7711273,"id":"11","nextBlock":-1,"opCode":"getArg","parameters":[],"spec":"type","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-11899692,"id":"24","nextBlock":25,"opCode":"setBgColor","parameters":["bar_1","0xFFFFFFFF"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"25","nextBlock":26,"opCode":"setBgColor","parameters":["bar_2","0xFFC0C000"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"26","nextBlock":27,"opCode":"setBgColor","parameters":["bar_3","0xFF00C0C0"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"27","nextBlock":28,"opCode":"setBgColor","parameters":["bar_4","0xFF00C000"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"28","nextBlock":29,"opCode":"setBgColor","parameters":["bar_5","0xFFC000C0"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"29","nextBlock":30,"opCode":"setBgColor","parameters":["bar_6","0xFFC00000"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"30","nextBlock":31,"opCode":"setBgColor","parameters":["bar_7","0xFF0000C0"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"31","nextBlock":-1,"opCode":"setBgColor","parameters":["bar_8","0xFF000000"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-1988310,"id":"13","nextBlock":55,"opCode":"if","parameters":["@14"],"spec":"if %b then","subStack1":16,"subStack2":-1,"type":"c","typeName":""}
{"color":-10701022,"id":"14","nextBlock":-1,"opCode":"stringEquals","parameters":["@15","transparent"],"spec":"%s equals %s","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-7711273,"id":"15","nextBlock":-1,"opCode":"getArg","parameters":[],"spec":"type","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-11899692,"id":"16","nextBlock":17,"opCode":"setBgColor","parameters":["bar_1","Color.TRANSPARENT"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"17","nextBlock":18,"opCode":"setBgColor","parameters":["bar_2","Color.TRANSPARENT"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"18","nextBlock":19,"opCode":"setBgColor","parameters":["bar_3","Color.TRANSPARENT"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"19","nextBlock":20,"opCode":"setBgColor","parameters":["bar_4","Color.TRANSPARENT"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"20","nextBlock":21,"opCode":"setBgColor","parameters":["bar_5","Color.TRANSPARENT"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"21","nextBlock":22,"opCode":"setBgColor","parameters":["bar_6","Color.TRANSPARENT"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"22","nextBlock":23,"opCode":"setBgColor","parameters":["bar_7","Color.TRANSPARENT"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"23","nextBlock":-1,"opCode":"setBgColor","parameters":["bar_8","Color.TRANSPARENT"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-1988310,"id":"55","nextBlock":32,"opCode":"if","parameters":["@56"],"spec":"if %b then","subStack1":66,"subStack2":-1,"type":"c","typeName":""}
{"color":-10701022,"id":"56","nextBlock":-1,"opCode":"stringEquals","parameters":["@57","blackbars"],"spec":"%s equals %s","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-7711273,"id":"57","nextBlock":-1,"opCode":"getArg","parameters":[],"spec":"type","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-13851166,"id":"66","nextBlock":-1,"opCode":"timerAfter","parameters":["time_switch","50"],"spec":"%m.timer after %d ms","subStack1":58,"subStack2":-1,"type":"c","typeName":""}
{"color":-11899692,"id":"58","nextBlock":59,"opCode":"setBgColor","parameters":["bar_1","0xFF000000"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"59","nextBlock":67,"opCode":"setBgColor","parameters":["bar_2","Color.TRANSPARENT"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"67","nextBlock":-1,"opCode":"timerAfter","parameters":["time_switch","50"],"spec":"%m.timer after %d ms","subStack1":68,"subStack2":-1,"type":"c","typeName":""}
{"color":-11899692,"id":"68","nextBlock":69,"opCode":"setBgColor","parameters":["bar_1","Color.TRANSPARENT"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"69","nextBlock":70,"opCode":"setBgColor","parameters":["bar_2","0xFF000000"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"70","nextBlock":-1,"opCode":"timerAfter","parameters":["time_switch","50"],"spec":"%m.timer after %d ms","subStack1":71,"subStack2":-1,"type":"c","typeName":""}
{"color":-11899692,"id":"71","nextBlock":72,"opCode":"setBgColor","parameters":["bar_3","0xFF000000"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"72","nextBlock":73,"opCode":"setBgColor","parameters":["bar_4","Color.TRANSPARENT"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"73","nextBlock":-1,"opCode":"timerAfter","parameters":["time_switch","50"],"spec":"%m.timer after %d ms","subStack1":74,"subStack2":-1,"type":"c","typeName":""}
{"color":-11899692,"id":"74","nextBlock":75,"opCode":"setBgColor","parameters":["bar_3","Color.TRANSPARENT"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"75","nextBlock":76,"opCode":"setBgColor","parameters":["bar_4","0xFF000000"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"76","nextBlock":-1,"opCode":"timerAfter","parameters":["time_switch","50"],"spec":"%m.timer after %d ms","subStack1":77,"subStack2":-1,"type":"c","typeName":""}
{"color":-11899692,"id":"77","nextBlock":78,"opCode":"setBgColor","parameters":["bar_5","0xFF000000"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"78","nextBlock":79,"opCode":"setBgColor","parameters":["bar_6","Color.TRANSPARENT"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"79","nextBlock":-1,"opCode":"timerAfter","parameters":["time_switch","50"],"spec":"%m.timer after %d ms","subStack1":80,"subStack2":-1,"type":"c","typeName":""}
{"color":-11899692,"id":"80","nextBlock":81,"opCode":"setBgColor","parameters":["bar_5","Color.TRANSPARENT"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"81","nextBlock":82,"opCode":"setBgColor","parameters":["bar_6","0xFF000000"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"82","nextBlock":-1,"opCode":"timerAfter","parameters":["time_switch","50"],"spec":"%m.timer after %d ms","subStack1":83,"subStack2":-1,"type":"c","typeName":""}
{"color":-11899692,"id":"83","nextBlock":84,"opCode":"setBgColor","parameters":["bar_7","0xFF000000"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"84","nextBlock":85,"opCode":"setBgColor","parameters":["bar_8","Color.TRANSPARENT"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"85","nextBlock":-1,"opCode":"timerAfter","parameters":["time_switch","50"],"spec":"%m.timer after %d ms","subStack1":86,"subStack2":-1,"type":"c","typeName":""}
{"color":-11899692,"id":"86","nextBlock":87,"opCode":"setBgColor","parameters":["bar_7","Color.TRANSPARENT"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"87","nextBlock":-1,"opCode":"setBgColor","parameters":["bar_8","0xFF000000"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-1988310,"id":"32","nextBlock":35,"opCode":"if","parameters":["@33"],"spec":"if %b then","subStack1":41,"subStack2":-1,"type":"c","typeName":""}
{"color":-10701022,"id":"33","nextBlock":-1,"opCode":"stringEquals","parameters":["@34","red"],"spec":"%s equals %s","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-7711273,"id":"34","nextBlock":-1,"opCode":"getArg","parameters":[],"spec":"type","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-11899692,"id":"41","nextBlock":-1,"opCode":"setBgColor","parameters":["background","0xFFC00000"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-1988310,"id":"35","nextBlock":38,"opCode":"if","parameters":["@36"],"spec":"if %b then","subStack1":42,"subStack2":-1,"type":"c","typeName":""}
{"color":-10701022,"id":"36","nextBlock":-1,"opCode":"stringEquals","parameters":["@37","green"],"spec":"%s equals %s","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-7711273,"id":"37","nextBlock":-1,"opCode":"getArg","parameters":[],"spec":"type","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-11899692,"id":"42","nextBlock":-1,"opCode":"setBgColor","parameters":["background","0xFF00C000"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-1988310,"id":"38","nextBlock":-1,"opCode":"if","parameters":["@39"],"spec":"if %b then","subStack1":43,"subStack2":-1,"type":"c","typeName":""}
{"color":-10701022,"id":"39","nextBlock":-1,"opCode":"stringEquals","parameters":["@40","blue"],"spec":"%s equals %s","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-7711273,"id":"40","nextBlock":-1,"opCode":"getArg","parameters":[],"spec":"type","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-11899692,"id":"43","nextBlock":-1,"opCode":"setBgColor","parameters":["background","0xFF0000C0"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@SpeakerCleanerActivity.java_onCreate_initializeLogic
{"color":-7711273,"id":"25","nextBlock":20,"opCode":"definedFunc","parameters":[],"spec":"font","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-1988310,"id":"20","nextBlock":10,"opCode":"ifElse","parameters":["@21"],"spec":"if %b then","subStack1":23,"subStack2":24,"type":"e","typeName":""}
{"color":-10701022,"id":"21","nextBlock":-1,"opCode":"stringEquals","parameters":["@22","dark"],"spec":"%s equals %s","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-13851166,"id":"22","nextBlock":-1,"opCode":"intentGetString","parameters":["theme"],"spec":"Activity getExtra key %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-7711273,"id":"23","nextBlock":-1,"opCode":"definedFunc","parameters":[],"spec":"DarkUI","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"24","nextBlock":-1,"opCode":"definedFunc","parameters":[],"spec":"LightUI","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"10","nextBlock":11,"opCode":"mediaplayerCreate","parameters":["cleaner","speaker_cleaner"],"spec":"%m.mediaplayer create %m.sound","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"11","nextBlock":12,"opCode":"mediaplayerStart","parameters":["cleaner"],"spec":"%m.mediaplayer start","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"12","nextBlock":-1,"opCode":"timerAfter","parameters":["timer","15000"],"spec":"%m.timer after %d ms","subStack1":16,"subStack2":-1,"type":"c","typeName":""}
{"color":-13851166,"id":"16","nextBlock":17,"opCode":"mediaplayerPause","parameters":["cleaner"],"spec":"%m.mediaplayer pause","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"17","nextBlock":18,"opCode":"mediaplayerReset","parameters":["cleaner"],"spec":"%m.mediaplayer reset","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"18","nextBlock":19,"opCode":"intentSetScreen","parameters":["back","DeviceinfoActivity"],"spec":"%m.intent setScreen %m.activity","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"19","nextBlock":-1,"opCode":"startActivity","parameters":["back"],"spec":"StartActivity %m.intent","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@SpeakerCleanerActivity.java_status_bar_color_moreBlock
{"color":-10701022,"id":"10","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["if (Build.VERSION.SDK_INT \u003e Build.VERSION_CODES.LOLLIPOP) { \n   Window w \u003d this.getWindow(); w.clearFlags(WindowManager.LayoutParams.FLAG_TRANSLUCENT_STATUS); w.addFlags(WindowManager.LayoutParams.FLAG_DRAWS_SYSTEM_BAR_BACKGROUNDS);\n   w.setStatusBarColor(Color.parseColor(_colour1)); w.setNavigationBarColor(Color.parseColor(_colour2));\n}"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@SpeakerCleanerActivity.java_LightStatusBar_moreBlock
{"color":-10701022,"id":"10","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["getWindow().getDecorView().setSystemUiVisibility(View.SYSTEM_UI_FLAG_LIGHT_STATUS_BAR);\r\ngetWindow().setStatusBarColor(0xFFFFFFFF);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@SpeakerCleanerActivity.java_LightUI_moreBlock
{"color":-7711273,"id":"17","nextBlock":10,"opCode":"definedFunc","parameters":[],"spec":"LightStatusBar","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"10","nextBlock":11,"opCode":"setBgColor","parameters":["linear_title","0xFFFFFFFF"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"11","nextBlock":12,"opCode":"setBgColor","parameters":["vscroll1","0xFFFFFFFF"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"12","nextBlock":13,"opCode":"setTextColor","parameters":["textview2","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"13","nextBlock":14,"opCode":"setTextColor","parameters":["show","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"14","nextBlock":15,"opCode":"setTextColor","parameters":["title","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"15","nextBlock":16,"opCode":"definedFunc","parameters":["#FFFFFF","#FFFFFF"],"spec":"status_bar_color %s.colour1 bottom_navigation_bar %s.colour2","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"16","nextBlock":-1,"opCode":"definedFunc","parameters":["linear7","#F2F3F5","#F2F3F5"],"spec":"Round %m.view.view Color1 %s.color1 Color2 %s.color2","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@SpeakerCleanerActivity.java_onResume_onResume
{"color":-13851166,"id":"10","nextBlock":-1,"opCode":"mediaplayerStart","parameters":["cleaner"],"spec":"%m.mediaplayer start","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@SpeakerCleanerActivity.java_DarkUI_moreBlock
{"color":-11899692,"id":"10","nextBlock":11,"opCode":"setBgColor","parameters":["linear_title","0xFF000000"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"11","nextBlock":12,"opCode":"setBgColor","parameters":["vscroll1","0xFF000000"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"12","nextBlock":13,"opCode":"setTextColor","parameters":["textview2","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"13","nextBlock":14,"opCode":"setTextColor","parameters":["show","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"14","nextBlock":15,"opCode":"setTextColor","parameters":["title","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"15","nextBlock":16,"opCode":"definedFunc","parameters":["#000000","#000000"],"spec":"status_bar_color %s.colour1 bottom_navigation_bar %s.colour2","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"16","nextBlock":-1,"opCode":"definedFunc","parameters":["linear7","#0D0C0A","#0D0C0A"],"spec":"Round %m.view.view Color1 %s.color1 Color2 %s.color2","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@SpeakerCleanerActivity.java_font_moreBlock
{"color":-11899692,"id":"11","nextBlock":10,"opCode":"setTypeface","parameters":["title","font_light",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"10","nextBlock":12,"opCode":"setTypeface","parameters":["textview2","font_light",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"12","nextBlock":-1,"opCode":"setTypeface","parameters":["show","font_regular",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@SpeakerCleanerActivity.java_onPause_onPause
{"color":-13851166,"id":"10","nextBlock":-1,"opCode":"mediaplayerPause","parameters":["cleaner"],"spec":"%m.mediaplayer pause","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@SpeakerCleanerActivity.java_Round_moreBlock
{"color":-10701022,"id":"10","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["int[] colors \u003d { Color.parseColor(_color1), Color.parseColor(_color2) }; android.graphics.drawable.GradientDrawable gd \u003d new android.graphics.drawable.GradientDrawable(android.graphics.drawable.GradientDrawable.Orientation.RIGHT_LEFT, colors); gd.setCornerRadius(50); _view.setBackground(gd);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@SpeakerCleanerActivity.java_onBackPressed_onBackPressed
{"color":-1147626,"id":"12","nextBlock":-1,"opCode":"setVarString","parameters":["useless","useless"],"spec":"set %m.varStr to %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@AboutActivity.java_facebook_onClick
{"color":-13851166,"id":"10","nextBlock":11,"opCode":"intentSetAction","parameters":["Link","ACTION_VIEW"],"spec":"%m.intent setAction %m.intentAction","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"11","nextBlock":12,"opCode":"intentSetData","parameters":["Link","https://www.facebook.com/itsme7studio"],"spec":"%m.intent setData %s.intentData","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"12","nextBlock":-1,"opCode":"startActivity","parameters":["Link"],"spec":"StartActivity %m.intent","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@AboutActivity.java_status_bar_color_moreBlock
{"color":-10701022,"id":"10","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["if (Build.VERSION.SDK_INT \u003e Build.VERSION_CODES.LOLLIPOP) { \n   Window w \u003d this.getWindow(); w.clearFlags(WindowManager.LayoutParams.FLAG_TRANSLUCENT_STATUS); w.addFlags(WindowManager.LayoutParams.FLAG_DRAWS_SYSTEM_BAR_BACKGROUNDS);\n   w.setStatusBarColor(Color.parseColor(_colour1)); w.setNavigationBarColor(Color.parseColor(_colour2));\n}"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@AboutActivity.java_Send_moreBlock
{"color":-13851166,"id":"10","nextBlock":11,"opCode":"intentSetAction","parameters":["@17","ACTION_VIEW"],"spec":"%m.intent setAction %m.intentAction","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"17","nextBlock":-1,"opCode":"getArg","parameters":[],"spec":"IntentName","subStack1":-1,"subStack2":-1,"type":"p","typeName":"Intent"}
{"color":-13851166,"id":"11","nextBlock":13,"opCode":"intentSetData","parameters":["@18","@22"],"spec":"%m.intent setData %s.intentData","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"18","nextBlock":-1,"opCode":"getArg","parameters":[],"spec":"IntentName","subStack1":-1,"subStack2":-1,"type":"p","typeName":"Intent"}
{"color":-10701022,"id":"22","nextBlock":-1,"opCode":"stringJoin","parameters":["mailto:","@23"],"spec":"join %s and %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-7711273,"id":"23","nextBlock":-1,"opCode":"getArg","parameters":[],"spec":"to","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-13851166,"id":"13","nextBlock":14,"opCode":"intentPutExtra","parameters":["@20","android.intent.extra.SUBJECT","@24"],"spec":"%m.intent putExtra key %s value %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"20","nextBlock":-1,"opCode":"getArg","parameters":[],"spec":"IntentName","subStack1":-1,"subStack2":-1,"type":"p","typeName":"Intent"}
{"color":-7711273,"id":"24","nextBlock":-1,"opCode":"getArg","parameters":[],"spec":"subject","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-13851166,"id":"14","nextBlock":15,"opCode":"intentPutExtra","parameters":["@19","android.intent.extra.TEXT","@25"],"spec":"%m.intent putExtra key %s value %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"19","nextBlock":-1,"opCode":"getArg","parameters":[],"spec":"IntentName","subStack1":-1,"subStack2":-1,"type":"p","typeName":"Intent"}
{"color":-7711273,"id":"25","nextBlock":-1,"opCode":"getArg","parameters":[],"spec":"text","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-13851166,"id":"15","nextBlock":-1,"opCode":"startActivity","parameters":["@16"],"spec":"StartActivity %m.intent","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"16","nextBlock":-1,"opCode":"getArg","parameters":[],"spec":"IntentName","subStack1":-1,"subStack2":-1,"type":"p","typeName":"Intent"}

@AboutActivity.java_whatsapp_onClick
{"color":-13851166,"id":"12","nextBlock":13,"opCode":"intentSetAction","parameters":["Link","ACTION_VIEW"],"spec":"%m.intent setAction %m.intentAction","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"13","nextBlock":11,"opCode":"intentSetData","parameters":["Link","https://wa.link/6pfoam"],"spec":"%m.intent setData %s.intentData","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"11","nextBlock":-1,"opCode":"startActivity","parameters":["Link"],"spec":"StartActivity %m.intent","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@AboutActivity.java_contact_me_theme_moreBlock
{"color":-1988310,"id":"10","nextBlock":11,"opCode":"if","parameters":["@13"],"spec":"if %b then","subStack1":21,"subStack2":-1,"type":"c","typeName":""}
{"color":-10701022,"id":"13","nextBlock":-1,"opCode":"\u003d","parameters":["@15","0"],"spec":"%d \u003d %d","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-7711273,"id":"15","nextBlock":-1,"opCode":"getArg","parameters":[],"spec":"theme","subStack1":-1,"subStack2":-1,"type":"d","typeName":""}
{"color":-11899692,"id":"21","nextBlock":22,"opCode":"setBgColor","parameters":["left","0xFF757575"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"22","nextBlock":23,"opCode":"setTextColor","parameters":["center","0xFF757575"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"23","nextBlock":24,"opCode":"setBgColor","parameters":["right","0xFF757575"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"24","nextBlock":25,"opCode":"definedFunc","parameters":["email","#F2F3F5","#F2F3F5"],"spec":"round_contact_me %m.view.view Color1 %s.color1 Color2 %s.color2","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"25","nextBlock":26,"opCode":"definedFunc","parameters":["facebook","#F2F3F5","#F2F3F5"],"spec":"round_contact_me %m.view.view Color1 %s.color1 Color2 %s.color2","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"26","nextBlock":33,"opCode":"definedFunc","parameters":["whatsapp","#F2F3F5","#F2F3F5"],"spec":"round_contact_me %m.view.view Color1 %s.color1 Color2 %s.color2","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"33","nextBlock":34,"opCode":"setImage","parameters":["email","ic_mail_black"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"34","nextBlock":35,"opCode":"setImage","parameters":["facebook","ic_facebook_black"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"35","nextBlock":-1,"opCode":"setImage","parameters":["whatsapp","ic_whatsapp_black"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-1988310,"id":"11","nextBlock":-1,"opCode":"if","parameters":["@14"],"spec":"if %b then","subStack1":18,"subStack2":-1,"type":"c","typeName":""}
{"color":-10701022,"id":"14","nextBlock":-1,"opCode":"\u003d","parameters":["@16","1"],"spec":"%d \u003d %d","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-7711273,"id":"16","nextBlock":-1,"opCode":"getArg","parameters":[],"spec":"theme","subStack1":-1,"subStack2":-1,"type":"d","typeName":""}
{"color":-11899692,"id":"18","nextBlock":19,"opCode":"setBgColor","parameters":["left","0xFF8A8A8A"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"19","nextBlock":20,"opCode":"setTextColor","parameters":["center","0xFF8A8A8A"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"20","nextBlock":27,"opCode":"setBgColor","parameters":["right","0xFF8A8A8A"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"27","nextBlock":28,"opCode":"definedFunc","parameters":["email","#0D0C0A","#0D0C0A"],"spec":"round_contact_me %m.view.view Color1 %s.color1 Color2 %s.color2","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"28","nextBlock":29,"opCode":"definedFunc","parameters":["facebook","#0D0C0A","#0D0C0A"],"spec":"round_contact_me %m.view.view Color1 %s.color1 Color2 %s.color2","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"29","nextBlock":30,"opCode":"definedFunc","parameters":["whatsapp","#0D0C0A","#0D0C0A"],"spec":"round_contact_me %m.view.view Color1 %s.color1 Color2 %s.color2","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"30","nextBlock":31,"opCode":"setImage","parameters":["email","ic_mail_white"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"31","nextBlock":32,"opCode":"setImage","parameters":["facebook","ic_facebook_white"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"32","nextBlock":-1,"opCode":"setImage","parameters":["whatsapp","ic_whatsapp_white"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@AboutActivity.java_ClickAnimation_moreBlock
{"color":-10701022,"id":"10","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["_view.setOnTouchListener(new View.OnTouchListener() {\n@Override\npublic boolean onTouch(View v, MotionEvent event) {\nswitch (event.getAction()){\ncase MotionEvent.ACTION_DOWN:{\nObjectAnimator scaleX \u003d new ObjectAnimator();\nscaleX.setTarget(_view);\nscaleX.setPropertyName(\"scaleX\");\nscaleX.setFloatValues(0.9f);\nscaleX.setDuration((int)_animDuration);\nscaleX.start();\n//Made by XenonDry\nObjectAnimator scaleY \u003d new ObjectAnimator();\nscaleY.setTarget(_view);\nscaleY.setPropertyName(\"scaleY\");\nscaleY.setFloatValues(0.9f);\nscaleY.setDuration((int)_animDuration);\nscaleY.start();\nbreak;\n}\ncase MotionEvent.ACTION_UP:{\n\nObjectAnimator scaleX \u003d new ObjectAnimator();\nscaleX.setTarget(_view);\nscaleX.setPropertyName(\"scaleX\");\nscaleX.setFloatValues((float)1);\nscaleX.setDuration((int)_animDuration);\nscaleX.start();\n\nObjectAnimator scaleY \u003d new ObjectAnimator();\nscaleY.setTarget(_view);\nscaleY.setPropertyName(\"scaleY\");\nscaleY.setFloatValues((float)1);\nscaleY.setDuration((int)_animDuration);\nscaleY.start();\n\nbreak;\n}\n}\nreturn false;\n}\n});"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@AboutActivity.java_onCreate_initializeLogic
{"color":-11899692,"id":"10","nextBlock":11,"opCode":"setTypeface","parameters":["title","font_light",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"11","nextBlock":12,"opCode":"setTypeface","parameters":["textview6","font_light",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"12","nextBlock":13,"opCode":"setTypeface","parameters":["textview1","font_regular",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"13","nextBlock":14,"opCode":"setTypeface","parameters":["textview2","font_light",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"14","nextBlock":53,"opCode":"setTypeface","parameters":["show_about","font_regular",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"53","nextBlock":16,"opCode":"setTypeface","parameters":["textview5","font_regular",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"16","nextBlock":17,"opCode":"setTypeface","parameters":["center","font_light",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-1988310,"id":"17","nextBlock":54,"opCode":"ifElse","parameters":["@18"],"spec":"if %b then","subStack1":51,"subStack2":52,"type":"e","typeName":""}
{"color":-10701022,"id":"18","nextBlock":-1,"opCode":"stringEquals","parameters":["@19","dark"],"spec":"%s equals %s","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-13851166,"id":"19","nextBlock":-1,"opCode":"intentGetString","parameters":["theme"],"spec":"Activity getExtra key %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-7711273,"id":"51","nextBlock":-1,"opCode":"definedFunc","parameters":[],"spec":"DarkUI","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"52","nextBlock":-1,"opCode":"definedFunc","parameters":[],"spec":"LightUI","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"54","nextBlock":55,"opCode":"addSourceDirectly","parameters":["//Used custom blocks made by XenonDry_THEB3IST on Sketchub"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"55","nextBlock":57,"opCode":"definedFunc","parameters":["@56","150","email"],"spec":"ClickAnimation %b.clickanim Duration %d.animDuration %m.view.view","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"56","nextBlock":-1,"opCode":"true","parameters":[],"spec":"true","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-7711273,"id":"57","nextBlock":59,"opCode":"definedFunc","parameters":["@58","150","facebook"],"spec":"ClickAnimation %b.clickanim Duration %d.animDuration %m.view.view","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"58","nextBlock":-1,"opCode":"true","parameters":[],"spec":"true","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-7711273,"id":"59","nextBlock":-1,"opCode":"definedFunc","parameters":["@60","150","whatsapp"],"spec":"ClickAnimation %b.clickanim Duration %d.animDuration %m.view.view","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"60","nextBlock":-1,"opCode":"true","parameters":[],"spec":"true","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}

@AboutActivity.java_email_onClick
{"color":-7711273,"id":"14","nextBlock":-1,"opCode":"definedFunc","parameters":["Link","sevenstudio@outlook.com.vn","",""],"spec":"Send Email %m.intent.IntentName To %s.to subject %s.subject text %s.text","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@AboutActivity.java_LightStatusBar_moreBlock
{"color":-10701022,"id":"10","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["getWindow().getDecorView().setSystemUiVisibility(View.SYSTEM_UI_FLAG_LIGHT_STATUS_BAR);\r\ngetWindow().setStatusBarColor(0xFFFFFFFF);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@AboutActivity.java_round_contact_me_moreBlock
{"color":-10701022,"id":"10","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["int[] colors \u003d { Color.parseColor(_color1), Color.parseColor(_color2) }; android.graphics.drawable.GradientDrawable gd \u003d new android.graphics.drawable.GradientDrawable(android.graphics.drawable.GradientDrawable.Orientation.RIGHT_LEFT, colors); gd.setCornerRadius(100); _view.setBackground(gd);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@AboutActivity.java_LightUI_moreBlock
{"color":-7711273,"id":"85","nextBlock":86,"opCode":"definedFunc","parameters":[],"spec":"LightStatusBar","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"86","nextBlock":102,"opCode":"definedFunc","parameters":["#FFFFFF","#FFFFFF"],"spec":"status_bar_color %s.colour1 bottom_navigation_bar %s.colour2","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"102","nextBlock":90,"opCode":"definedFunc","parameters":["0"],"spec":"contact_me_theme %d.theme","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"90","nextBlock":91,"opCode":"setBgColor","parameters":["linear_title","0xFFFFFFFF"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"91","nextBlock":94,"opCode":"setBgColor","parameters":["vscroll1","0xFFFFFFFF"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"94","nextBlock":95,"opCode":"setTextColor","parameters":["title","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"95","nextBlock":96,"opCode":"setTextColor","parameters":["textview6","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"96","nextBlock":97,"opCode":"setTextColor","parameters":["textview1","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"97","nextBlock":98,"opCode":"setTextColor","parameters":["show_about","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"98","nextBlock":101,"opCode":"setTextColor","parameters":["textview1","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"101","nextBlock":-1,"opCode":"setTextColor","parameters":["textview5","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@AboutActivity.java_DarkUI_moreBlock
{"color":-7711273,"id":"85","nextBlock":101,"opCode":"definedFunc","parameters":["#000000","#000000"],"spec":"status_bar_color %s.colour1 bottom_navigation_bar %s.colour2","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"101","nextBlock":102,"opCode":"definedFunc","parameters":["1"],"spec":"contact_me_theme %d.theme","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"102","nextBlock":103,"opCode":"definedFunc","parameters":["email","#0D0C0A","#0D0C0A"],"spec":"round_contact_me %m.view.view Color1 %s.color1 Color2 %s.color2","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"103","nextBlock":104,"opCode":"definedFunc","parameters":["facebook","#0D0C0A","#0D0C0A"],"spec":"round_contact_me %m.view.view Color1 %s.color1 Color2 %s.color2","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"104","nextBlock":89,"opCode":"definedFunc","parameters":["whatsapp","#0D0C0A","#0D0C0A"],"spec":"round_contact_me %m.view.view Color1 %s.color1 Color2 %s.color2","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"89","nextBlock":90,"opCode":"setBgColor","parameters":["linear_title","0xFF000000"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"90","nextBlock":93,"opCode":"setBgColor","parameters":["vscroll1","0xFF000000"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"93","nextBlock":94,"opCode":"setTextColor","parameters":["title","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"94","nextBlock":95,"opCode":"setTextColor","parameters":["textview6","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"95","nextBlock":96,"opCode":"setTextColor","parameters":["textview1","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"96","nextBlock":97,"opCode":"setTextColor","parameters":["show_about","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"97","nextBlock":100,"opCode":"setTextColor","parameters":["textview2","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"100","nextBlock":-1,"opCode":"setTextColor","parameters":["textview5","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@AboutActivity.java_RoundAndBorder_moreBlock
{"color":-10701022,"id":"10","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["android.graphics.drawable.GradientDrawable gd \u003d new android.graphics.drawable.GradientDrawable();\ngd.setColor(Color.parseColor(_color1));\ngd.setCornerRadius((int) _round);\ngd.setStroke((int) _border, Color.parseColor(_color2));\n_view.setBackground(gd);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@AboutActivity.java_Round_moreBlock
{"color":-10701022,"id":"10","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["int[] colors \u003d { Color.parseColor(_color1), Color.parseColor(_color2) }; android.graphics.drawable.GradientDrawable gd \u003d new android.graphics.drawable.GradientDrawable(android.graphics.drawable.GradientDrawable.Orientation.RIGHT_LEFT, colors); gd.setCornerRadius(50); _view.setBackground(gd);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@ToolsActivity.java_onCreate_initializeLogic
{"color":-7711273,"id":"14","nextBlock":15,"opCode":"definedFunc","parameters":[],"spec":"font","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-1988310,"id":"15","nextBlock":22,"opCode":"ifElse","parameters":["@16"],"spec":"if %b then","subStack1":18,"subStack2":19,"type":"e","typeName":""}
{"color":-10701022,"id":"16","nextBlock":-1,"opCode":"stringEquals","parameters":["@17","dark"],"spec":"%s equals %s","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-13851166,"id":"17","nextBlock":-1,"opCode":"intentGetString","parameters":["theme"],"spec":"Activity getExtra key %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-7711273,"id":"18","nextBlock":-1,"opCode":"definedFunc","parameters":[],"spec":"DarkUI","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"19","nextBlock":-1,"opCode":"definedFunc","parameters":[],"spec":"LightUI","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"22","nextBlock":20,"opCode":"addSourceDirectly","parameters":["//Used custom blocks made by XenonDry_THEB3IST on Sketchub"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"20","nextBlock":21,"opCode":"definedFunc","parameters":["@23","150","linear7"],"spec":"ClickAnimation %b.clickanim Duration %d.animDuration %m.view.view","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"23","nextBlock":-1,"opCode":"true","parameters":[],"spec":"true","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-7711273,"id":"21","nextBlock":-1,"opCode":"definedFunc","parameters":["@24","150","linear10"],"spec":"ClickAnimation %b.clickanim Duration %d.animDuration %m.view.view","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"24","nextBlock":-1,"opCode":"true","parameters":[],"spec":"true","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}

@ToolsActivity.java_status_bar_color_moreBlock
{"color":-10701022,"id":"10","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["if (Build.VERSION.SDK_INT \u003e Build.VERSION_CODES.LOLLIPOP) { \n   Window w \u003d this.getWindow(); w.clearFlags(WindowManager.LayoutParams.FLAG_TRANSLUCENT_STATUS); w.addFlags(WindowManager.LayoutParams.FLAG_DRAWS_SYSTEM_BAR_BACKGROUNDS);\n   w.setStatusBarColor(Color.parseColor(_colour1)); w.setNavigationBarColor(Color.parseColor(_colour2));\n}"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@ToolsActivity.java_LightStatusBar_moreBlock
{"color":-10701022,"id":"10","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["getWindow().getDecorView().setSystemUiVisibility(View.SYSTEM_UI_FLAG_LIGHT_STATUS_BAR);\r\ngetWindow().setStatusBarColor(0xFFFFFFFF);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@ToolsActivity.java_linear10_onClick
{"color":-13851166,"id":"10","nextBlock":11,"opCode":"intentSetScreen","parameters":["go","DisplayTestActivity"],"spec":"%m.intent setScreen %m.activity","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"11","nextBlock":-1,"opCode":"startActivity","parameters":["go"],"spec":"StartActivity %m.intent","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@ToolsActivity.java_LightUI_moreBlock
{"color":-7711273,"id":"17","nextBlock":10,"opCode":"definedFunc","parameters":[],"spec":"LightStatusBar","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"10","nextBlock":11,"opCode":"setBgColor","parameters":["linear_title","0xFFFFFFFF"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"11","nextBlock":12,"opCode":"setBgColor","parameters":["vscroll1","0xFFFFFFFF"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"12","nextBlock":13,"opCode":"setTextColor","parameters":["textview2","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"13","nextBlock":14,"opCode":"setTextColor","parameters":["show","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"14","nextBlock":24,"opCode":"setTextColor","parameters":["title","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"24","nextBlock":25,"opCode":"setTextColor","parameters":["textview5","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"25","nextBlock":26,"opCode":"setTextColor","parameters":["textview6","0xFF000000"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"26","nextBlock":27,"opCode":"setImage","parameters":["imageview1","ic_fix_speaker"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"27","nextBlock":15,"opCode":"setImage","parameters":["imageview3","ic_fix_display"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"15","nextBlock":16,"opCode":"definedFunc","parameters":["#FFFFFF","#FFFFFF"],"spec":"status_bar_color %s.colour1 bottom_navigation_bar %s.colour2","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"16","nextBlock":18,"opCode":"definedFunc","parameters":["linear7","#F2F3F5","#F2F3F5"],"spec":"Round %m.view.view Color1 %s.color1 Color2 %s.color2","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"18","nextBlock":-1,"opCode":"definedFunc","parameters":["linear10","#F2F3F5","#F2F3F5"],"spec":"Round %m.view.view Color1 %s.color1 Color2 %s.color2","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@ToolsActivity.java_DarkUI_moreBlock
{"color":-11899692,"id":"10","nextBlock":11,"opCode":"setBgColor","parameters":["linear_title","0xFF000000"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"11","nextBlock":12,"opCode":"setBgColor","parameters":["vscroll1","0xFF000000"],"spec":"%m.view setBackgroundColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"12","nextBlock":13,"opCode":"setTextColor","parameters":["textview2","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"13","nextBlock":14,"opCode":"setTextColor","parameters":["show","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"14","nextBlock":18,"opCode":"setTextColor","parameters":["title","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"18","nextBlock":19,"opCode":"setTextColor","parameters":["textview5","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"19","nextBlock":20,"opCode":"setTextColor","parameters":["textview6","0xFFFFFFFF"],"spec":"%m.textview setTextColor %m.color","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"20","nextBlock":21,"opCode":"setImage","parameters":["imageview1","ic_fix_speaker"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"21","nextBlock":15,"opCode":"setImage","parameters":["imageview3","ic_fix_display"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"15","nextBlock":16,"opCode":"definedFunc","parameters":["#000000","#000000"],"spec":"status_bar_color %s.colour1 bottom_navigation_bar %s.colour2","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"16","nextBlock":17,"opCode":"definedFunc","parameters":["linear7","#0D0C0A","#0D0C0A"],"spec":"Round %m.view.view Color1 %s.color1 Color2 %s.color2","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-7711273,"id":"17","nextBlock":-1,"opCode":"definedFunc","parameters":["linear10","#0D0C0A","#0D0C0A"],"spec":"Round %m.view.view Color1 %s.color1 Color2 %s.color2","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@ToolsActivity.java_font_moreBlock
{"color":-11899692,"id":"10","nextBlock":12,"opCode":"setTypeface","parameters":["title","font_light",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"12","nextBlock":11,"opCode":"setTypeface","parameters":["textview2","font_light",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"11","nextBlock":13,"opCode":"setTypeface","parameters":["textview5","font_light",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"13","nextBlock":14,"opCode":"setTypeface","parameters":["show","font_regular",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"14","nextBlock":-1,"opCode":"setTypeface","parameters":["textview6","font_regular",""],"spec":"%m.textview setTypeface %m.font with style %m.typeface","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@ToolsActivity.java_linear7_onClick
{"color":-1988310,"id":"12","nextBlock":-1,"opCode":"ifElse","parameters":["@13"],"spec":"if %b then","subStack1":15,"subStack2":18,"type":"e","typeName":""}
{"color":-10701022,"id":"13","nextBlock":-1,"opCode":"stringEquals","parameters":["@21","dark"],"spec":"%s equals %s","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-13851166,"id":"21","nextBlock":-1,"opCode":"intentGetString","parameters":["theme"],"spec":"Activity getExtra key %s","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-13851166,"id":"15","nextBlock":16,"opCode":"intentPutExtra","parameters":["go","theme","dark"],"spec":"%m.intent putExtra key %s value %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"16","nextBlock":17,"opCode":"intentSetScreen","parameters":["go","SpeakerCleanerActivity"],"spec":"%m.intent setScreen %m.activity","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"17","nextBlock":-1,"opCode":"startActivity","parameters":["go"],"spec":"StartActivity %m.intent","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"18","nextBlock":19,"opCode":"intentPutExtra","parameters":["go","theme","light"],"spec":"%m.intent putExtra key %s value %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"19","nextBlock":20,"opCode":"intentSetScreen","parameters":["go","SpeakerCleanerActivity"],"spec":"%m.intent setScreen %m.activity","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-13851166,"id":"20","nextBlock":-1,"opCode":"startActivity","parameters":["go"],"spec":"StartActivity %m.intent","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@ToolsActivity.java_Round_moreBlock
{"color":-10701022,"id":"10","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["int[] colors \u003d { Color.parseColor(_color1), Color.parseColor(_color2) }; android.graphics.drawable.GradientDrawable gd \u003d new android.graphics.drawable.GradientDrawable(android.graphics.drawable.GradientDrawable.Orientation.RIGHT_LEFT, colors); gd.setCornerRadius(50); _view.setBackground(gd);"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}

@ToolsActivity.java_ClickAnimation_moreBlock
{"color":-10701022,"id":"10","nextBlock":-1,"opCode":"addSourceDirectly","parameters":["_view.setOnTouchListener(new View.OnTouchListener() {\n@Override\npublic boolean onTouch(View v, MotionEvent event) {\nswitch (event.getAction()){\ncase MotionEvent.ACTION_DOWN:{\nObjectAnimator scaleX \u003d new ObjectAnimator();\nscaleX.setTarget(_view);\nscaleX.setPropertyName(\"scaleX\");\nscaleX.setFloatValues(0.9f);\nscaleX.setDuration((int)_animDuration);\nscaleX.start();\n//Made by XenonDry\nObjectAnimator scaleY \u003d new ObjectAnimator();\nscaleY.setTarget(_view);\nscaleY.setPropertyName(\"scaleY\");\nscaleY.setFloatValues(0.9f);\nscaleY.setDuration((int)_animDuration);\nscaleY.start();\nbreak;\n}\ncase MotionEvent.ACTION_UP:{\n\nObjectAnimator scaleX \u003d new ObjectAnimator();\nscaleX.setTarget(_view);\nscaleX.setPropertyName(\"scaleX\");\nscaleX.setFloatValues((float)1);\nscaleX.setDuration((int)_animDuration);\nscaleX.start();\n\nObjectAnimator scaleY \u003d new ObjectAnimator();\nscaleY.setTarget(_view);\nscaleY.setPropertyName(\"scaleY\");\nscaleY.setFloatValues((float)1);\nscaleY.setDuration((int)_animDuration);\nscaleY.start();\n\nbreak;\n}\n}\nreturn false;\n}\n});"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}"##;

    match Logic::parse(input) {
        Ok(r) => r,
        Err(err) => panic!("Failed to parse logic: {}", err)
    };

    // sadly i can't check if it produces a correct output, its just tooo huge lmao
}