import React from 'react';
import { useState } from 'react';

interface MobileMenuProps {
    activeButton: number;
    setActiveButton: (buttonIndex: number) => void;
  }

  const MobileMenu: React.FC<MobileMenuProps> = ({ activeButton, setActiveButton }) => {
    // const [activeButton, setActiveButton] = useState(0);
    const handleButtonClick = (buttonIndex: number) => {
        setActiveButton(buttonIndex);
    };
    // const handleSignOut = async () => {

    //     const supabase = createClient();
    //     await supabase.auth.signOut();
    // }
    //@ts-ignore
    // const { user } = useAuth();
    return (
        // <div className="rounded-box btm-nav top-0 z-10 fixed flex justify-around">
        <div className="btm-nav fixed flex justify-around z-10 ">
            <button
                className={`text-primary ${activeButton === 1 ? 'active' : ''}`}
                onClick={() => handleButtonClick(1)}
            >
                <svg xmlns="http://www.w3.org/2000/svg" className="h-7 w-7" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
                </svg>
                Integrations
            </button>
            <button
                className={` text-primary ${activeButton === 2 ? 'active' : ''}`}
                onClick={() => handleButtonClick(2)}
            >
                <svg xmlns="http://www.w3.org/2000/svg" className="h-7 w-7" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
                Wallet
            </button>
            <button
                className={`text-primary ${activeButton === 3 ? 'active' : ''}`}
                onClick={() => handleButtonClick(3)}
            >
                <svg xmlns="http://www.w3.org/2000/svg" className="h-7 w-7" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
                </svg>
                Settings
            </button>
        </div>
    )
}

export default MobileMenu;