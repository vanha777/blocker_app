import React from 'react';
import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api'


const Loading = ({ setUser, setLoading, setConfig }) => {
    const [progress, setProgress] = useState(0);

    useEffect(() => {
        let interval;
        const callEndpoint = async () => {
            try {
                // Simulate the progress increment
                interval = setInterval(() => {
                    setProgress((prev) => {
                        if (prev >= 100) {
                            clearInterval(interval);
                            return 100;
                        }
                        return prev + 1;
                    });
                }, 100); // Increment every 100ms

                const response = await invoke("read_config").then((response) => {
                    setProgress(90);
                    console.log("Debug:This is read_config response ", response);
                    setTimeout(() => {
                        clearInterval(interval);
                        setProgress(100);
                        setLoading(false);
                        setConfig(response)
                        if (response.session_id != null) {
                            setUser(true);
                        } else {
                            setUser(false);
                        }
                    }, 2000); // 2-second delay
                });
            } catch (error) {
                console.error(error);
                clearInterval(interval);
                // setLoading(true); // Set loading to true even if there is an error
            }
        };
        callEndpoint();
        // Clean up the interval on component unmount
        return () => clearInterval(interval);
    }, []); // Empty dependency array ensures this runs only once

    const callEndpoint = () => {
        invoke("fetch_data").then((response) => setMessage(response))
    };

    return (
        <div>
            <div
                className="radial-progress text-primary"
                style={{ "--value": progress, "--size": "12rem", "--thickness": "2rem" }}
                role="progressbar"
            >
                {progress}%
            </div>
        </div>
    );
}

export default Loading;