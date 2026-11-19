import PublishedNewsletters from './PublishedNewsletters';
import PublishNewsletter from './PublishNewsletter';

const NewslettersTab = () => {
  return (
    <div className="w-full h-full mt-4 px-4 grid grid-cols-2 gap-4">
      <PublishedNewsletters />
      <PublishNewsletter />
    </div>
  );
};

export default NewslettersTab;
