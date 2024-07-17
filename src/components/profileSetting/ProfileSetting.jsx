import moment from "moment"
import { useEffect, useState } from "react"
// import { useDispatch, useSelector } from "react-redux"
import TitleCard from "../TitleCard"
// import { showNotification } from '../../common/headerSlice'
import InputText from './InputText'
import TextAreaInput from './TextAreaInput'
import ToogleInput from './ToogleInput'

function ProfileSettings({config}) {

    const [isSync, setIsSync] = useState(false);


    // const dispatch = useDispatch()

    // Call API to update profile settings changes
    const updateProfile = () => {
        // dispatch(showNotification({message : "Profile Updated", status : 1}))    
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
                    <TextAreaInput labelTitle="Api support" defaultValue={config.api_config} updateFormValue={updateFormValue} updateType="api_config" />
                </div>
                <div className="divider" ></div>

                <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                    {/* <InputText labelTitle="Language" defaultValue="English" updateFormValue={updateFormValue}/>
                    <InputText labelTitle="Timezone" defaultValue="IST" updateFormValue={updateFormValue}/> */}
                    {/* <ToogleInput updateType="syncData" labelTitle="Sync Data" defaultValue={true} updateFormValue={updateFormValue}/> */}
                    <p1>Sync Data</p1>
                    <input type="checkbox" className="toggle toggle-success toggle-lg" checked={isSync} onChange={() => updateSyncStatus()} />
                </div>

                <div className="mt-16"><button className="btn btn-primary float-right" onClick={() => updateProfile()}>Update</button></div>
            </TitleCard>
        </>
    )
}


export default ProfileSettings