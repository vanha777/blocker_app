// Sidebar.js
import React from 'react';

const Sidebar = () => {
  return (
    <div className="flex flex-col w-64 h-screen bg-base-100 text-white">
      <div className="flex items-center justify-center h-20 shadow-md">
        <h1 className="text-2xl font-bold">Logo</h1>
      </div>
      <div className="flex flex-col flex-1 overflow-y-auto">
        <nav className="flex-1 px-4 py-2 space-y-1">
          <a href="#" className="block px-4 py-2 text-sm font-semibold text-gray-300 rounded-md hover:bg-gray-700">
            Dashboard
          </a>
          <a href="#" className="block px-4 py-2 text-sm font-semibold text-gray-300 rounded-md hover:bg-gray-700">
            Profile
          </a>
          <a href="#" className="block px-4 py-2 text-sm font-semibold text-gray-300 rounded-md hover:bg-gray-700">
            Settings
          </a>
          <a href="#" className="block px-4 py-2 text-sm font-semibold text-gray-300 rounded-md hover:bg-gray-700">
            Logout
          </a>
        </nav>
      </div>
    </div>
  );
};

export default Sidebar;
