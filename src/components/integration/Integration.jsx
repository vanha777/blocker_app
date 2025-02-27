import { useState, useEffect } from "react"
// import { useDispatch } from "react-redux"
import TitleCard from "../TitleCard"
import { invoke } from '@tauri-apps/api'
// import { showNotification } from "./headerSlice"


// const INITIAL_INTEGRATION_LIST = [
//     {name : "Slack", icon : "https://eazypic.s3.ap-southeast-4.amazonaws.com/Image_17-7-2024_at_11.30_PM-removebg-preview.png", isActive : true, description : "Slack is an instant messaging program designed by Slack Technologies and owned by Salesforce."},
//     {name : "Facebook", icon : "https://cdn-icons-png.flaticon.com/512/124/124010.png", isActive : false, description : "Meta Platforms, Inc., doing business as Meta and formerly named Facebook, Inc., and TheFacebook."},
//     {name : "Linkedin", icon : "https://cdn-icons-png.flaticon.com/512/174/174857.png", isActive : true, description : "LinkedIn is a business and employment-focused social media platform that works through websites and mobile apps."},
//     {name : "Google Ads", icon : "https://cdn-icons-png.flaticon.com/512/2301/2301145.png", isActive : false, description : "Google Ads is an online advertising platform developed by Google, where advertisers bid to display brief advertisements, service offerings"},
//     {name : "Gmail", icon : "https://cdn-icons-png.flaticon.com/512/5968/5968534.png", isActive : false, description : "Gmail is a free email service provided by Google. As of 2019, it had 1.5 billion active users worldwide."},
//     {name : "Salesforce", icon : "https://cdn-icons-png.flaticon.com/512/5968/5968880.png", isActive : false, description : "It provides customer relationship management software and applications focused on sales, customer service, marketing automation."},
//     {name : "Hubspot", icon : "https://cdn-icons-png.flaticon.com/512/5968/5968872.png", isActive : false, description : "American developer and marketer of software products for inbound marketing, sales, and customer service."},
// ]

function Integration({ config, setConfig }) {

    useEffect(() => {
        console.log("Config updated detected ...");
        invoke("read_config").then((response) => {
            setConfig(response);
        }).catch((err) => {
            console.log("error to fetch config ", err);
        });
    }, []); // Empty dependency array ensures this runs only once

    const [integrationList, setIntegrationList] = useState(config.api_config || null);
    const [newConfig, setNewConfig] = useState(config);
    const updateIntegrationStatus = (index) => {
        console.log("changing status integrations status ", index);
        let integration = integrationList[index]
        setIntegrationList(integrationList.map((i, k) => {
            if (k === index) return { ...i, isActive: !i.isActive }
            return i
        }))
        // console.log("this is updated ", integrationList[index])
    }

    useEffect(() => {
        let new_config = config;
        new_config.api_config = integrationList;
            invoke("config_edit", { config: new_config }).then((response) => {
                console.log("this is response config_update ", response);
                // setConfig(response)
            })
    }, [integrationList]);

    return (
        <>
            <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
                {integrationList != null ? (
                    integrationList.map((i, k) => {
                        return (
                            <TitleCard key={k} title={i.name} topMargin={"mt-2"}>
                                <p className="flex">
                                    <img alt="icon" src={`data:image/png;base64,${i.icon}`} className="w-12 h-12 inline-block mr-4" />
                                    {i.description}
                                </p>
                                <div className="mt-6 text-right">
                                    <input type="checkbox" className="toggle toggle-success toggle-lg"  style={
                                        { '--tglbg': '#D9C6E0' ,border:"transparent"}
                                        } checked={i.isActive} onChange={() => updateIntegrationStatus(k)} />
                                </div>
                            </TitleCard>
                        )
                    }
                    )
                ) : (
                    <p>No API data available</p>
                )
                }
            </div>
        </>
    )
}
export default Integration