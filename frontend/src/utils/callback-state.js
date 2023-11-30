import { useRef, useState, useEffect } from "react";

const useCallbackState = (value) => {
  const [state, setState] = useState(value);
  const callback = useRef();

  const updateState = (newVal, cb) => {
    setState(newVal);
    callback.current = cb;
  };

  useEffect(() => {
    if (callback.current) {
      callback.current(state);
    }
  }, [state]);

  return [state, updateState];
};

export { useCallbackState };
