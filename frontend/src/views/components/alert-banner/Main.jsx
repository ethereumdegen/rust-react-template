import React, { useState, useEffect } from "react";

const AlertBanner = ({ type, message }) => {
  const [show, setShow] = useState(false);

  useEffect(() => {
    if (message) {
      setShow(true);
      const timer = setTimeout(() => {
        setShow(false);
      }, 15000);
      return () => clearTimeout(timer);
    }
  }, [message]);

  if (!show) {
    return null;
  }

  return (
    <div
      className={`overflow-x-auto ${
        type === "warning"
          ? "bg-yellow-100 text-yellow-900"
          : "bg-red-100 text-red-900"
      } border border-solid border-${
        type === "warning" ? "yellow" : "red"
      }-500 rounded-md px-4 py-3 mb-4`}
      role="alert"
    >
      <p className="font-bold">{type === "warning" ? "Warning" : "Error"}</p>
      <p className="text-sm">{message}</p>
    </div>
  );
};

export default AlertBanner;
