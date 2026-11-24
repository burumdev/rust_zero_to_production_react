import { useRef } from 'react';
import { v7 as uuidv7 } from 'uuid';

const useIdempotencyKey = () => {
  const keyRef = useRef<string | null>(null);

  const getOrCreateKey = () => {
    if (!keyRef.current) {
      keyRef.current = uuidv7();
    }

    return keyRef.current;
  };

  const clearKey = () => {
    keyRef.current = null;
  };

  const rotateKey = () => {
    keyRef.current = uuidv7();
    return keyRef.current;
  };

  return {
    getOrCreateKey,
    clearKey,
    rotateKey,
  };
};

export default useIdempotencyKey;
