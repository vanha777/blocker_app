import moment from "moment"
import { useEffect, useState } from "react"
// import { useDispatch, useSelector } from "react-redux"
import TitleCard from "../TitleCard"
// import { showNotification } from '../../common/headerSlice'
import InputText from './InputText'
import TextAreaInput from './TextAreaInput'
import ToogleInput from './ToogleInput'
import { invoke } from '@tauri-apps/api'

function ProfileSettings({ config,setConfig }) {

    const [isSync, setIsSync] = useState(false);


    // const dispatch = useDispatch()

    // Call API to update profile settings changes
    const updateProfile = () => {
        invoke("config_update", { config:config}).then((response) => {
            console.log("Config update ",response);
            setConfig(response);
        }).catch((error) => {
            console.error('Error invoking read_config:', error);
        });
    }

    const updateFormValue = ({ updateType, value }) => {
        console.log(value)
        console.log(updateType)
    }

    const updateSyncStatus = () => {
        // call BE update config file 
        setIsSync(!isSync);
    }

    return (
        <>
            <TitleCard title="Configuration" topMargin="mt-2">
                <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                    <InputText labelTitle="Session Id" defaultValue={config.session_id} updateFormValue={updateFormValue} updateType="session_id" />
                    <InputText labelTitle="Cloud Url" defaultValue={config.cloud_url} updateFormValue={updateFormValue} updateType="cloud_url" />
                    <InputText labelTitle="Client Id" defaultValue={config.client_id} updateFormValue={updateFormValue} updateType="client_id" />
                    <InputText labelTitle="Version" defaultValue={config.version} updateFormValue={updateFormValue} updateType="version" />
                </div>
                <div className="divider" ></div>
                <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
                    {
                        config.api_config.map((i, k) => {
                            return (
                                <>
                                    <InputText labelTitle="Name" defaultValue={i.name} updateFormValue={updateFormValue} updateType="name" />
                                    <InputText labelTitle="Api Key" defaultValue={i.api_key} updateFormValue={updateFormValue} updateType="api_key" />
                                    <InputText labelTitle="Subscription Key" defaultValue={i.subscription_key} updateFormValue={updateFormValue} updateType="subscription_key" />
                                </>
                            )
                        })
                    }
                </div>
                <div className="mt-16"><button className="btn btn-primary float-right" onClick={() => updateProfile()}>Update</button></div>
            </TitleCard>
        </>
    )
}

export default ProfileSettings