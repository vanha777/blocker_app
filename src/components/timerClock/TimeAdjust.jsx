import moment from "moment"
import { useEffect, useState } from "react"
import { invoke } from '@tauri-apps/api'

function TimerAdjust({ time, setTime }) {
    //teseting
    const [base64Image, setBase64Image] = useState();
    //end.
    const [hours, setHours] = useState();
    const [minutes, setMinutes] = useState();
    const [seconds, setSeconds] = useState();
    // Generate an array of numbers from 0 to 10
    const Defaulthours = Array.from({ length: 11 }, (_, i) => i);
    // Generate an array of numbers from 0 to 60
    const Defaultminutes = Array.from({ length: 61 }, (_, i) => i);
    // Generate an array of numbers from 0 to 60
    const Defaultseconds = Array.from({ length: 61 }, (_, i) => i);

    const handleAccept = (e) => {
        e.preventDefault();
        console.log(`Selected time: ${hours} hours, ${minutes} minutes, ${seconds} seconds`);
        const totalSeconds = (parseInt(hours) * 3600) + (parseInt(minutes) * 60) + parseInt(seconds);
        setTime(totalSeconds);
        // Close the dialog
        const dialog = document.getElementById('my_modal_4');
        dialog.close();
    };


    useEffect(() => {
        let clock = formatTime(time);
        setHours(clock.hours);
        setMinutes(clock.minutes);
        setSeconds(clock.seconds);
    }, [time]);

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

    const setNewTime = (time) => {
        const hours = String(Math.floor(time / 3600)).padStart(2, '0'); // Calculate hours
        const minutes = String(Math.floor((time % 3600) / 60)).padStart(2, '0'); // Calculate remaining minutes
        const seconds = String(time % 60).padStart(2, '0'); // Calculate remaining seconds

        return {
            hours,
            minutes,
            seconds
        };
    };

    return (
        <dialog id="my_modal_4" className="modal text-white">
            <div
                className="modal-box"
                style={{
                    height: '38rem',
                    maxWidth: '640px',
                    background: 'linear-gradient(to right, rgba(230, 169, 228, 0.5), rgba(155, 201, 255, 0.5))',
                }}
            >

                <div class="grid grid-cols-2 gap-4">
                    <label for="hoursSelect" class="font-semibold">Hours:</label>
                    <select className="select select-ghost w-full max-w-xs bg-transparent" onChange={(e) => setHours(e.target.value)}>
                        <option disabled selected>Pick a number</option>
                        {Defaulthours.map(num => (
                            <option key={num} value={num}>{num}</option>
                        ))}
                    </select>


                    <label for="minutesSelect" class="font-semibold">Minutes:</label>
                    <select className="select select-ghost w-full max-w-xs bg-transparent" onChange={(e) => setMinutes(e.target.value)}>
                        <option disabled selected>Pick a number</option>
                        {Defaultminutes.map(num => (
                            <option key={num} value={num}>{num}</option>
                        ))}
                    </select>

                    <label for="secondsSelect" class="font-semibold">Seconds:</label>
                    <select className="select select-ghost w-full max-w-xs bg-transparent" onChange={(e) => setSeconds(e.target.value)}>
                        <option disabled selected>Pick a number</option>
                        {Defaultseconds.map(num => (
                            <option key={num} value={num}>{num}</option>
                        ))}
                    </select>

                </div>

                <div className="modal-action">
                    <form method="dialog">
                        <button className="btn" onClick={(e) => handleAccept(e)}>Accept</button>
                    </form>
                 
                </div>

            </div>

        </dialog>
    )
}

export default TimerAdjust