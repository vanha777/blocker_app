import moment from "moment"
import { useEffect, useState } from "react"
// import { useDispatch, useSelector } from "react-redux"
import TitleCard from "../TitleCard"
// import { showNotification } from '../../common/headerSlice'
import InputText from './InputText'
import TextAreaInput from './TextAreaInput'
import ToogleInput from './ToogleInput'
import { invoke } from '@tauri-apps/api'

function ProfileSettings({ config, setConfig, setLoading }) {

    // const [isSync, setIsSync] = useState(false);
    // const [newConfig, setNewConfig] = useState(config);

    // useEffect(() => {
    //     console.log("Config updated detected ...");
    //     invoke("read_config").then((response) => {
    //         setNewConfig(config);
    //     }).catch((err) => {
    //         console.log("error to fetch config ", err);
    //     });
    // }, []); // Empty dependency array ensures this runs only once


    // const dispatch = useDispatch()

    // Call API to update profile settings changes
    const updateProfile = () => {
        console.log("updating config file ...")
        invoke("config_update", { config: config }).then((response) => {
            console.log("Config update ", response);
            setLoading(true);
        }).catch((error) => {
            console.error('Error invoking read_config:', error);
        });
    }

    const updateFormValue = ({ updateType, value }) => {
        console.log(value)
        console.log(updateType)
    }

    return (
        <>
            <TitleCard title="Configuration" topMargin="mt-2">
                <div className="grid grid-cols-1 md:grid-cols-2 gap-6">

                    <div className="form-control w-full">
                        <label className="label">
                            <span className="label-text text-base-content ">Session Id</span>
                        </label>
                        <input disabled type="text" value={config.session_id} placeholder="" className="input  input-bordered w-full " />
                    </div>
                    <div className="form-control w-full">
                        <label className="label">
                            <span className="label-text text-base-content ">Cloud Url</span>
                        </label>
                        <input disabled type="text" value={config.cloud_url} placeholder="" className="input  input-bordered w-full " />
                    </div>
                    <div className="form-control w-full">
                        <label className="label">
                            <span className="label-text text-base-content ">Client Id</span>
                        </label>
                        <input disabled type="text" value={config.client_id} placeholder="" className="input  input-bordered w-full " />
                    </div>
                    <div className="form-control w-full">
                        <label className="label">
                            <span className="label-text text-base-content ">Version</span>
                        </label>
                        <input disabled type="text" value={config.version} placeholder="" className="input  input-bordered w-full " />
                    </div>

                    {/* <InputText labelTitle="Session Id" defaultValue={config.session_id} updateFormValue={updateFormValue} updateType="session_id" />
                    <InputText labelTitle="Cloud Url" defaultValue={config.cloud_url} updateFormValue={updateFormValue} updateType="cloud_url" />
                    <InputText labelTitle="Client Id" defaultValue={config.client_id} updateFormValue={updateFormValue} updateType="client_id" />
                    <InputText labelTitle="Version" defaultValue={config.version} updateType="version" /> */}
                </div>
                <div className="divider" ></div>
                <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
                    {
                        config.api_config ? (
                            config.api_config.map((i, k) => {
                                return (
                                    <>
                                        <div className="form-control w-full">
                                            <label className="label">
                                                <span className="label-text text-base-content ">Name</span>
                                            </label>
                                            <input disabled type="text" value={i.integration_name} placeholder="" className="input  input-bordered w-full " />
                                        </div>
                                        <div className="form-control w-full">
                                            <label className="label">
                                                <span className="label-text text-base-content ">Api Key</span>
                                            </label>
                                            <input disabled type="text" value={i.api_key} placeholder="" className="input  input-bordered w-full " />
                                        </div>
                                        <div className="form-control w-full">
                                            <label className="label">
                                                <span className="label-text text-base-content ">Subscription Key</span>
                                            </label>
                                            <input disabled type="text" value={i.subscription_key} placeholder="" className="input  input-bordered w-full " />
                                        </div>
                                        {/* <InputText labelTitle="Name" defaultValue={i.integration_name} updateFormValue={updateFormValue} updateType="name" />
                                        <InputText labelTitle="Api Key" defaultValue={i.api_key} updateFormValue={updateFormValue} updateType="api_key" />
                                        <InputText labelTitle="Subscription Key" defaultValue={i.subscription_key} updateFormValue={updateFormValue} updateType="subscription_key" /> */}
                                    </>
                                )
                            })
                        ) : (
                            <p>No API configurations found.</p>
                        )
                    }
                </div>

                {/* <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
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
                </div> */}
                <div className="mt-16"><button className="btn btn-primary float-right" onClick={() => updateProfile()}>Update</button></div>
            </TitleCard>
        </>
    )
}

export default ProfileSettings