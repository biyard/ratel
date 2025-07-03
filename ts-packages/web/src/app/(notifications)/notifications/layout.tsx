import { ArrowRight } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';
import Image from 'next/image';
import { peopleToFollow, articles } from './data';

export default function Layout({ children }: { children: React.ReactNode }) {
  return (
    <div className="max-w-7xl mx-auto px-6 py-6">
      <div className="grid grid-cols-3 gap-6">
        {children}
        {/* Right Column - Sidebar */}
        <div className="space-y-6">
          {/* Hot Promotion */}
          <div className="bg-component-bg rounded-lg p-4">
            <div className="flex items-center justify-between mb-4">
              <h3 className="font-semibold text-white">Hot Promotion</h3>
            </div>
            <div className="flex items-center gap-3 mb-3">
              <Image
                src="/images/poll.png"
                width={100}
                height={100}
                alt="Poll image"
              />
              <div>
                <h4 className="font-medium text-white">
                  Presidential Election
                </h4>
              </div>
            </div>
            <button className="text-neutral-500 text-sm flex items-center gap-1 hover:text-white">
              View all <ArrowRight className="w-3 h-3" />
            </button>
          </div>

          {/* News */}
          <div className="space-y-4 bg-component-bg p-4">
            <h3 className="font-semibold text-white">News</h3>

            <div className="space-y-4">
              {articles.map((article) => (
                <div key={article.id} className="space-y-2">
                  <h4 className="font-medium text-sm text-white">
                    {article.title}
                  </h4>
                  <p className="text-neutral-500 text-xs">
                    {article.description}
                  </p>
                  <button className="text-neutral-500 text-xs flex items-center gap-1 hover:text-white">
                    View Detail <ArrowRight className="w-3 h-3" />
                  </button>
                </div>
              ))}
            </div>
          </div>

          {/* Add to your feed */}
          <div className="space-y-4 bg-component-bg p-4">
            <h3 className="font-semibold text-white">Add to your feed</h3>

            <div className="space-y-4">
              {peopleToFollow.map((person) => (
                <div key={person.id} className="flex items-center gap-3">
                  <Avatar className="w-10 h-10">
                    <AvatarImage src={person.image} />
                    <AvatarFallback>{person.fallback}</AvatarFallback>
                  </Avatar>
                  <div className="flex-1">
                    <h4 className="font-medium text-sm text-white">
                      {person.name}
                    </h4>
                    <p className="text-neutral-500 text-xs">{person.role}</p>
                  </div>
                  <Button
                    size="sm"
                    variant="outline"
                    className="bg-neutral-700 border-neutral-700 text-white hover:bg-[#262626]"
                  >
                    + Follow
                  </Button>
                </div>
              ))}
            </div>

            <button className="text-neutral-500 text-sm flex items-center gap-1 hover:text-white">
              View all <ArrowRight className="w-3 h-3" />
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
