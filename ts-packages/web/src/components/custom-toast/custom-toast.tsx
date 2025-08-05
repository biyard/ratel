import React from 'react';


type CustomToastProps = {
  message: string;
}

const CustomToast: React.FC<CustomToastProps> = ({ message }) => (
  <div style={{
    backgroundColor: '#007BFF',
    color: '#fff',
    display: 'flex',
    alignItems: 'center',
    padding: '10px 15px',
    borderRadius: '5px'
  }}>
  
    <span>{message}</span>
  </div>
);

export default CustomToast;
