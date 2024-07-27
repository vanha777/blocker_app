import moment from "moment"
import { useEffect, useState } from "react"
import { invoke } from '@tauri-apps/api'
import TimerAdjust from "./TimeAdjust";

function TimerClock({ config, setConfig, setLoading }) {
    // const numbers = Array.from({ length: 100 }, (_, index) => index);

    const [time, setTime] = useState(300); // Set initial time in seconds (e.g., 5 minutes)
    const [isRunning, setIsRunning] = useState(false);
    // const [adjustment, setAdjustment] = useState(false);

    useEffect(() => {
        let timer;
        if (isRunning && time > 0) {
            timer = setInterval(() => {
                setTime((prevTime) => prevTime - 1);
            }, 1000);
        } else if (!isRunning && time !== 0) {
            clearInterval(timer);
        }
        return () => clearInterval(timer);
    }, [isRunning, time]);

    const formatTime = (time) => {
        const hours = String(Math.floor(time / 3600)).padStart(2, '0'); // Calculate hours
        const minutes = String(Math.floor((time % 3600) / 60)).padStart(2, '0'); // Calculate remaining minutes
        const seconds = String(time % 60).padStart(2, '0'); // Calculate remaining seconds

        return {
            hours,
            minutes,
            seconds
        };
    };

    const handleCheckboxChange = () => {
        setIsRunning((prev) => !prev);
    };
    const handleClick = () => {
        console.log('Div clicked!');
        document.getElementById('my_modal_4').showModal()
    };

    return (
        // <div className="card bg-base-300 w-96 shadow-xl relative">
        //     <div className=" text-white absolute top-2 right-2 bg-transparent">XS</div>
        //     <div className="card-body items-center text-center">
        //         {/* <h2 className="card-title">Shoes!</h2> */}
        //         {/* <h2 className="card-title text-5xl font-bold" style="font-family: 'Orbitron', sans-serif;"> */}
        //         <h2 className="card-title text-7xl font-bold text-white">
        //             12:34
        //         </h2>
        //         <label className="btn btn-lg btn-active btn-ghost bg-black swap swap-rotate w-64">
        //             {/* this hidden checkbox controls the state */}
        //             <input type="checkbox" />
        //             <svg className="swap-off fill-white" fill="none" height="32" stroke="white" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" viewBox="0 0 24 24" width="32" xmlns="http://www.w3.org/2000/svg"><polygon points="5 3 19 12 5 21 5 3" /></svg>
        //             <svg className="swap-on fill-white" fill="none" height="32" stroke="white" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" viewBox="0 0 24 24" width="32" xmlns="http://www.w3.org/2000/svg"><rect height="16" width="4" x="6" y="4" /><rect height="16" width="4" x="14" y="4" /></svg>
        //         </label>
        //         {/* <input type="range" min={0} max="100" value="40" className="range [--range-shdw:black]" /> */}
        //     </div>
        // </div>
        <>
            <div className="card bg-transparent w-96 relative mt-[-250px]"
                style={{
                    height: '270px',
                    width: '600px',
                }}
            >
                {/* <div className="card-title">
                sas
            <div className="text-white absolute top-2 right-2 bg-transparent">XS</div>
            </div> */}

                <div className="text-white absolute top-2 right-2 bg-transparent">XS</div>

                <div className="card-body items-center text-center" >
                    {/* <button className="btn-ghost bg-transparent " disabled={isRunning} onClick={handleClick}>
                        <h2 className="card-title text-9xl font-bold text-white" >
                            {formatTime(time)}
                        </h2>
                    </button> */}

                    <button className="card-title btn-ghost bg-transparent " disabled={isRunning} onClick={handleClick}>
                        <span className="countdown font-mono text-8xl text-white">
                            <span style={{ "--value": formatTime(time).hours }}></span>:
                            <span style={{ "--value": formatTime(time).minutes }}></span>:
                            <span style={{ "--value": formatTime(time).seconds }}></span>
                        </span>
                    </button>

                    {/* <select className="select select-lg select-ghost w-full max-w-xs bg-transparent">
                    <option disabled selected>Pick your favorite Simpson</option>
                    {numbers.map((character, index) => (
                        <option key={index} value={character}>{character}</option>
                    ))}
                </select> */}
                    <label className="btn btn-lg btn-ghost bg-black swap swap-rotate w-64"
                        style={{
                            // height: '600px',
                            // width: '100%',
                            // maxWidth: '1280px',
                            background: 'linear-gradient(to right, rgba(155, 201, 255, 0.5), rgba(230, 169, 228, 0.5))',
                        }}>
                        <input type="checkbox" onChange={handleCheckboxChange} />
                        <svg className="swap-off fill-white" fill="none" height="32" stroke="white" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" viewBox="0 0 24 24" width="32" xmlns="http://www.w3.org/2000/svg"><polygon points="5 3 19 12 5 21 5 3" /></svg>
                        <svg className="swap-on fill-white" fill="none" height="32" stroke="white" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" viewBox="0 0 24 24" width="32" xmlns="http://www.w3.org/2000/svg"><rect height="16" width="4" x="6" y="4" /><rect height="16" width="4" x="14" y="4" /></svg>
                    </label>
                </div>
            </div >

            <TimerAdjust time={time} setTime={setTime} id="my_modal_4"/>

        </>
    )
}

export default TimerClock