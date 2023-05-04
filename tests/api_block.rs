use swrs::api::block::Blocks;
use swrs::parser::logic::BlockContainer;
use swrs::parser::Parsable;

#[test]
fn test1() {
    let logic = r#"{"color":-1988310,"id":"14","nextBlock":25,"opCode":"ifElse","parameters":["@19"],"spec":"if %b then","subStack1":40,"subStack2":43,"type":"e","typeName":""}
{"color":-10701022,"id":"19","nextBlock":-1,"opCode":"\u003e","parameters":["@20","0"],"spec":"%d \u003e %d","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-3384542,"id":"20","nextBlock":-1,"opCode":"lengthList","parameters":["webviews"],"spec":"length of %m.list","subStack1":-1,"subStack2":-1,"type":"d","typeName":""}
{"color":-11899692,"id":"40","nextBlock":41,"opCode":"setVisible","parameters":["linear_notab","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"41","nextBlock":-1,"opCode":"setVisible","parameters":["listview1","VISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"43","nextBlock":42,"opCode":"setVisible","parameters":["listview1","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"42","nextBlock":-1,"opCode":"setVisible","parameters":["linear_notab","VISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-1988310,"id":"25","nextBlock":-1,"opCode":"ifElse","parameters":["@26"],"spec":"if %b then","subStack1":21,"subStack2":36,"type":"e","typeName":""}
{"color":-10701022,"id":"26","nextBlock":-1,"opCode":"\u003e","parameters":["@27","99"],"spec":"%d \u003e %d","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
{"color":-3384542,"id":"27","nextBlock":-1,"opCode":"lengthList","parameters":["webviews"],"spec":"length of %m.list","subStack1":-1,"subStack2":-1,"type":"d","typeName":""}
{"color":-11899692,"id":"21","nextBlock":39,"opCode":"setText","parameters":["textview_tabs",":D"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"39","nextBlock":-1,"opCode":"setText","parameters":["textview1",":D"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-11899692,"id":"36","nextBlock":15,"opCode":"setText","parameters":["textview_tabs","@37"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"37","nextBlock":-1,"opCode":"toString","parameters":["@38"],"spec":"toString %d without decimal","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-3384542,"id":"38","nextBlock":-1,"opCode":"lengthList","parameters":["webviews"],"spec":"length of %m.list","subStack1":-1,"subStack2":-1,"type":"d","typeName":""}
{"color":-11899692,"id":"15","nextBlock":-1,"opCode":"setText","parameters":["textview1","@17"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
{"color":-10701022,"id":"17","nextBlock":-1,"opCode":"toString","parameters":["@18"],"spec":"toString %d without decimal","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
{"color":-3384542,"id":"18","nextBlock":-1,"opCode":"lengthList","parameters":["webviews"],"spec":"length of %m.list","subStack1":-1,"subStack2":-1,"type":"d","typeName":""}"#;

    // {"color":-10701022,"id":"24","nextBlock":20,"opCode":"addSourceDirectly","parameters":["// Detect if there\u0027s any webview that being displayed"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
    // {"color":-1988310,"id":"20","nextBlock":-1,"opCode":"if","parameters":["@21"],"spec":"if %b then","subStack1":25,"subStack2":-1,"type":"c","typeName":""}
    // {"color":-10701022,"id":"21","nextBlock":-1,"opCode":"not","parameters":["@22"],"spec":"not %b","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
    // {"color":-10701022,"id":"22","nextBlock":-1,"opCode":"\u003d","parameters":["@23","-1"],"spec":"%d \u003d %d","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
    // {"color":-1147626,"id":"23","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"currentWV","subStack1":-1,"subStack2":-1,"type":"d","typeName":""}
    // {"color":-10701022,"id":"25","nextBlock":11,"opCode":"addSourceDirectly","parameters":["// Detect if the current webview is loaded or not"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
    // {"color":-1988310,"id":"11","nextBlock":-1,"opCode":"ifElse","parameters":["@12"],"spec":"if %b then","subStack1":26,"subStack2":27,"type":"e","typeName":""}
    // {"color":-1147626,"id":"12","nextBlock":-1,"opCode":"getVar","parameters":[],"spec":"isLoaded","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
    // {"color":-10701022,"id":"26","nextBlock":10,"opCode":"addSourceDirectly","parameters":["// Re-load the web content"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
    // {"color":-10701022,"id":"10","nextBlock":15,"opCode":"addSourceDirectly","parameters":["((WebView)webviews.get((int)currentWV).get(\"webview\")).reload();"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
    // {"color":-11899692,"id":"15","nextBlock":17,"opCode":"setImage","parameters":["imageview2","ic_clear_white"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
    // {"color":-1147626,"id":"17","nextBlock":-1,"opCode":"setVarBoolean","parameters":["isLoaded","@19"],"spec":"set %m.varBool to %b","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
    // {"color":-10701022,"id":"19","nextBlock":-1,"opCode":"false","parameters":[],"spec":"false","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
    // {"color":-10701022,"id":"27","nextBlock":13,"opCode":"addSourceDirectly","parameters":["// Stop the content from loading"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
    // {"color":-10701022,"id":"13","nextBlock":14,"opCode":"addSourceDirectly","parameters":["((WebView)webviews.get((int)currentWV).get(\"webview\")).stopLoading();"],"spec":"add source directly %s.inputOnly","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
    // {"color":-11899692,"id":"14","nextBlock":16,"opCode":"setImage","parameters":["imageview2","ic_refresh_white"],"spec":"%m.imageview setImage %m.resource","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
    // {"color":-1147626,"id":"16","nextBlock":-1,"opCode":"setVarBoolean","parameters":["isLoaded","@18"],"spec":"set %m.varBool to %b","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
    // {"color":-10701022,"id":"18","nextBlock":-1,"opCode":"true","parameters":[],"spec":"true","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
    //
    // @MainActivity.java_refreshTabsCount_moreBlock
    // {"color":-1988310,"id":"14","nextBlock":25,"opCode":"ifElse","parameters":["@19"],"spec":"if %b then","subStack1":40,"subStack2":43,"type":"e","typeName":""}
    // {"color":-10701022,"id":"19","nextBlock":-1,"opCode":"\u003e","parameters":["@20","0"],"spec":"%d \u003e %d","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
    // {"color":-3384542,"id":"20","nextBlock":-1,"opCode":"lengthList","parameters":["webviews"],"spec":"length of %m.list","subStack1":-1,"subStack2":-1,"type":"d","typeName":""}
    // {"color":-11899692,"id":"40","nextBlock":41,"opCode":"setVisible","parameters":["linear_notab","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
    // {"color":-11899692,"id":"41","nextBlock":-1,"opCode":"setVisible","parameters":["listview1","VISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
    // {"color":-11899692,"id":"43","nextBlock":42,"opCode":"setVisible","parameters":["listview1","GONE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
    // {"color":-11899692,"id":"42","nextBlock":-1,"opCode":"setVisible","parameters":["linear_notab","VISIBLE"],"spec":"%m.view setVisible %m.visible","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
    // {"color":-1988310,"id":"25","nextBlock":-1,"opCode":"ifElse","parameters":["@26"],"spec":"if %b then","subStack1":21,"subStack2":36,"type":"e","typeName":""}
    // {"color":-10701022,"id":"26","nextBlock":-1,"opCode":"\u003e","parameters":["@27","99"],"spec":"%d \u003e %d","subStack1":-1,"subStack2":-1,"type":"b","typeName":""}
    // {"color":-3384542,"id":"27","nextBlock":-1,"opCode":"lengthList","parameters":["webviews"],"spec":"length of %m.list","subStack1":-1,"subStack2":-1,"type":"d","typeName":""}
    // {"color":-11899692,"id":"21","nextBlock":39,"opCode":"setText","parameters":["textview_tabs",":D"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
    // {"color":-11899692,"id":"39","nextBlock":-1,"opCode":"setText","parameters":["textview1",":D"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
    // {"color":-11899692,"id":"36","nextBlock":15,"opCode":"setText","parameters":["textview_tabs","@37"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
    // {"color":-10701022,"id":"37","nextBlock":-1,"opCode":"toString","parameters":["@38"],"spec":"toString %d without decimal","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
    // {"color":-3384542,"id":"38","nextBlock":-1,"opCode":"lengthList","parameters":["webviews"],"spec":"length of %m.list","subStack1":-1,"subStack2":-1,"type":"d","typeName":""}
    // {"color":-11899692,"id":"15","nextBlock":-1,"opCode":"setText","parameters":["textview1","@17"],"spec":"%m.textview setText %s","subStack1":-1,"subStack2":-1,"type":" ","typeName":""}
    // {"color":-10701022,"id":"17","nextBlock":-1,"opCode":"toString","parameters":["@18"],"spec":"toString %d without decimal","subStack1":-1,"subStack2":-1,"type":"s","typeName":""}
    // {"color":-3384542,"id":"18","nextBlock":-1,"opCode":"lengthList","parameters":["webviews"],"spec":"length of %m.list","subStack1":-1,"subStack2":-1,"type":"d","typeName":""}

    let pblocks = BlockContainer::parse(logic).unwrap();
    let blocks = Blocks::try_from(pblocks.clone()).unwrap();

    fn print_blocks(blocks: Blocks, depth: usize) {
        for block in blocks {
            print!("{}", "| |   ".repeat(depth));
            println!(
                "[ {} ]: {} ({:?})",
                block.content.to_string(),
                block.op_code,
                block.category().unwrap()
            );

            if let Some(ss1) = block.sub_stack1 {
                print_blocks(ss1, depth + 1);
                print!("{}", "| |   ".repeat(depth));
                println!("[        ]");
            }

            if let Some(ss2) = block.sub_stack2 {
                print_blocks(ss2, depth + 1);
                print!("{}", "| |   ".repeat(depth));
                println!("[      ]");
            }
        }
    }

    print_blocks(blocks.clone(), 0);

    let rpblocks: BlockContainer = blocks.into();
    for rpblock in rpblocks.0 {
        println!("{:?}", rpblock);
    }

    // assert_eq!(rpblocks, pblocks); // Blocks in this new API doesn't preserve the IDs, so
    //                                   it definitely wont match one-per-one
}
