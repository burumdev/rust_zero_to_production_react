import { useState } from 'react';

import Modal from '@components/Modal';
import NewsletterModal from '@user/components/NewsletterModal';

import './App.css';

const App = () => {
  const [isNewsletterModalOpen, setNewsletterModalOpen] = useState(true);

  return (
    <>
      <div className="flex flex-col items-center justify-center">
        <img className="-mt-[20px]" src="/images/logo.svg" />
        <p className="w-[640px] italic text-center text-sm">
          A place where dreams turn into nested,
          <br />
          self-referential nightmares.
          <br />
          But you don't remember them when you wake up.
        </p>
      </div>
      <Modal
        isOpen={isNewsletterModalOpen}
        onClose={() => setNewsletterModalOpen(false)}
      >
        <NewsletterModal />
      </Modal>
    </>
  );
};

export default App;
