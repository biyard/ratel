'use client';

import {
  MessageCircle,
  RotateCcw,
  MoreHorizontal,
  Warehouse,
  Phone,
  Video,
  Search,
  MoreVertical,
  Archive,
  X,
  Star,
  Heart,
  Clock,
} from 'lucide-react';
import { Input } from '@/components/ui/input';
import { Avatar, AvatarFallback } from '@/components/ui/avatar';
import { useState } from 'react';
import { notifications } from './data';
import { Button } from '@/components/ui/button';

export default function NotificationClientPage() {
  const [activeTab, setActiveTab] = useState<'notification' | 'message'>(
    'notification',
  );
  const [selectedUser, setSelectedUser] = useState<string | null>(null);

  const handleUserClick = (userName: string) => {
    setSelectedUser(userName);
  };

  const handleCloseConversation = () => {
    setSelectedUser(null);
  };

  return (
    <div className="col-span-2 space-y-4">
      {/* Tab Headers */}
      <div className="bg px-6 py-4 rounded-t-lg">
        <div className="grid grid-cols-2 gap-0">
          <div className="flex justify-center">
            <button
              onClick={() => setActiveTab('notification')}
              className={`text-xl font-semibold transition-colors relative ${
                activeTab === 'notification'
                  ? 'text-white'
                  : 'text-neutral-500 hover:text-white'
              }`}
            >
              Notification
              {activeTab === 'notification' && (
                <div className="absolute -bottom-4 left-1/2 transform -translate-x-1/2 w-12 h-1 bg-primary rounded-full"></div>
              )}
            </button>
          </div>
          <div className="flex justify-center">
            <button
              onClick={() => setActiveTab('message')}
              className={`text-xl font-semibold transition-colors relative ${
                activeTab === 'message'
                  ? 'text-white'
                  : 'text-neutral-500 hover:text-white'
              }`}
            >
              Message
              {activeTab === 'message' && (
                <div className="absolute -bottom-4 left-1/2 transform -translate-x-1/2 w-12 h-1 bg-primary rounded-full"></div>
              )}
            </button>
          </div>
        </div>
      </div>

      {/* Tab Content */}
      {activeTab === 'notification' && (
        <div className="px-6 py-4 rounded-b-lg space-y-6 bg-component-bg">
          {/* Filter Tabs */}
          <div className="bg-neutral-800 rounded-full p-1 flex justify-between items-center text-sm w-full">
            <button className="bg-white text-black px-6 py-2 rounded-full font-medium">
              All
            </button>
            <button className="text-neutral-500 hover:text-white px-4 py-2 flex items-center gap-2 transition-colors">
              <RotateCcw className="w-4 h-4" />
              Replies
            </button>
            <button className="text-neutral-500 hover:text-white px-4 py-2 flex items-center gap-2 transition-colors">
              <span className="text-base">@</span>
              Mention
            </button>
            <button className="text-neutral-500 hover:text-white px-4 py-2 flex items-center gap-2 transition-colors">
              <Warehouse className="w-4 h-4" />
              Spaces
            </button>
          </div>

          {/* Notification Items */}
          {notifications.map((item) => (
            <div
              key={item.id}
              className="flex items-start gap-3 p-3 hover:bg rounded-lg"
            >
              {item.type === 'User' ? (
                <Avatar className="w-10 h-10">
                  <AvatarFallback className="bg-white text-neutral-500">
                    {item.icon}
                  </AvatarFallback>
                </Avatar>
              ) : (
                <div className="w-10 h-10 bg-white rounded-lg flex items-center justify-center">
                  <span className="text-xs text-neutral-500">{item.icon}</span>
                </div>
              )}

              <div className="flex-1">
                <div className="flex items-center gap-2">
                  <span className="font-medium text-white">{item.title}</span>
                  <span className="text-neutral-500 text-sm">
                    {item.message}
                  </span>
                </div>
                <p className="text-neutral-500 text-sm">{item.description}</p>
              </div>

              <div className="flex items-center gap-2">
                <span className="text-neutral-500 text-xs">{item.timeAgo}</span>
                <MoreHorizontal className="w-4 h-4 text-neutral-500" />
              </div>
            </div>
          ))}
        </div>
      )}

      {activeTab === 'message' && (
        <div className="px-6 py-4 rounded-b-lg space-y-6 bg-component-bg">
          {/* Search Bar and Call Icons Row */}
          <div className="flex items-center justify-between mb-4">
            <div className="relative flex-1 max-w-md">
              <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-neutral-500 w-4 h-4" />
              <Input
                placeholder="Search"
                className="pl-10 bg-neutral-800 border-neutral-700 text-white placeholder-neutral-500 w-full rounded-full"
              />
            </div>
            <div className="flex items-center gap-4">
              <Phone className="w-5 h-5 text-neutral-500 cursor-pointer hover:text-white" />
              <Video className="w-5 h-5 text-neutral-500 cursor-pointer hover:text-white" />
              <MessageCircle className="w-5 h-5 text-neutral-500 cursor-pointer hover:text-white" />
              <MoreVertical className="w-5 h-5 text-neutral-500 cursor-pointer hover:text-white" />
            </div>
          </div>

          {/* Filter Tabs Row - Full Width with Corrected Background */}
          <div className="bg-neutral-800 rounded-full p-1 flex justify-between items-center text-sm w-full">
            <button className="bg-white text-black px-6 py-2 rounded-full font-medium whitespace-nowrap">
              All
            </button>
            <button className="text-neutral-500 hover:text-white px-4 py-2 whitespace-nowrap">
              Unread
            </button>
            <button className="text-neutral-500 hover:text-white px-4 py-2 whitespace-nowrap">
              My Connections
            </button>
            <button className="text-neutral-500 hover:text-white px-4 py-2 whitespace-nowrap">
              Other
            </button>
            <button className="text-neutral-500 hover:text-white px-4 py-2 whitespace-nowrap">
              Archived
            </button>
            <button className="text-neutral-500 hover:text-white px-2 py-2">
              <MoreHorizontal className="w-4 h-4" />
            </button>
          </div>

          {/* Content Area */}
          <div className="grid grid-cols-2 gap-6 h-[calc(100vh-400px)]">
            {/* Notification Section */}
            <div className="flex flex-col h-full">
              {/* Notification List */}
              <div className="flex-1 overflow-y-auto space-y-3 pr-2 scrollbar-thin scrollbar-thumb-gray-600 scrollbar-track-transparent">
                <div className="space-y-3">
                  {Array.from({ length: 8 }, (_, i) => (
                    <div
                      key={i}
                      className="flex items-start gap-3 p-2 hover:bg-neutral-800 rounded-lg transition-colors cursor-pointer group"
                      onClick={() => handleUserClick(`User ${i + 1}`)}
                    >
                      <Avatar className="w-10 h-10 flex-shrink-0">
                        <AvatarFallback className="bg-neutral-700 text-white text-sm">
                          U
                        </AvatarFallback>
                      </Avatar>
                      <div className="flex-1 min-w-0">
                        <div className="flex items-start justify-between gap-2 mb-1">
                          <p className="font-medium text-white text-sm group-hover:text-primary transition-colors cursor-pointer">
                            [User name]
                          </p>
                          <span className="text-neutral-500 text-xs whitespace-nowrap">
                            12hrs ago
                          </span>
                        </div>
                        <p className="text-neutral-500 text-xs group-hover:text-neutral-400 transition-colors">
                          Hey Where's your computer scienc...
                        </p>
                      </div>
                    </div>
                  ))}
                </div>
              </div>

              {/* Message Section */}
            </div>

            {/* Message Section */}
            <div className="flex flex-col h-full">
              {selectedUser ? (
                /* Detailed Conversation View - Contained */
                <div className="flex flex-col h-full overflow-hidden">
                  {/* Conversation Header */}
                  <div className="flex items-center justify-between p-4 bg-neutral-800 rounded-t-lg flex-shrink-0">
                    <div className="flex items-center gap-3">
                      <Avatar className="w-10 h-10">
                        <AvatarFallback className="bg-neutral-700 text-white text-sm">
                          U
                        </AvatarFallback>
                      </Avatar>
                      <span className="font-medium text-white">
                        [User name]
                      </span>
                    </div>
                    <div className="flex items-center gap-4">
                      <Phone className="w-5 h-5 text-neutral-500 cursor-pointer hover:text-white" />
                      <Video className="w-5 h-5 text-neutral-500 cursor-pointer hover:text-white" />
                      <Archive className="w-5 h-5 text-neutral-500 cursor-pointer hover:text-white" />
                      <MoreVertical className="w-5 h-5 text-neutral-500 cursor-pointer hover:text-white" />
                      <X
                        className="w-5 h-5 text-neutral-500 cursor-pointer hover:text-white"
                        onClick={handleCloseConversation}
                      />
                    </div>
                  </div>

                  {/* Scrollable Content Area */}
                  <div className="flex-1 overflow-y-auto p-4 space-y-4 bg-[#1e1e1e] rounded-b-lg">
                    {/* Post Content */}
                    <div className="bg-neutral-700 rounded-lg p-4 space-y-4">
                      <div className="flex items-center justify-between">
                        <h3 className="font-semibold text-white">
                          [Post Title]
                        </h3>
                        <span className="text-neutral-500 text-xs">1w ago</span>
                      </div>

                      <div className="flex items-center gap-2">
                        <Avatar className="w-6 h-6">
                          <AvatarFallback className="bg-primary text-black text-xs">
                            P
                          </AvatarFallback>
                        </Avatar>
                        <span className="text-sm text-white">
                          Politician name
                        </span>
                        <Star className="w-4 h-4 text-primary fill-current" />
                      </div>

                      <p className="text-white text-sm leading-relaxed">
                        Life isn't a straight road, and it's not supposed to be.
                        Some turns teach you patience, some dead ends build your
                        strength. It's not always about moving fast—it's about
                        moving with meaning. Even when you feel lost, you're
                        gathering pieces of yourself along the way. Every
                        mistake, every delay, every unexpected moment is shaping
                        a version of you that's wiser, kinder, and more real.
                        You don't need to have it all figured out. You just need
                        to keep showing up for yourself, one honest step at a
                        time.
                      </p>

                      <div className="flex items-center gap-4">
                        <div className="flex items-center gap-2">
                          <Avatar className="w-6 h-6">
                            <AvatarFallback className="bg-primary text-black text-xs">
                              R
                            </AvatarFallback>
                          </Avatar>
                          <span className="text-sm text-white">
                            It's our place!
                          </span>
                        </div>
                        <div className="flex items-center gap-3 ml-auto">
                          <button className="flex items-center gap-1 text-neutral-500 hover:text-red-500 transition-colors">
                            <Heart className="w-4 h-4" />
                            <span className="text-xs">1</span>
                          </button>
                          <button className="text-neutral-500 hover:text-white transition-colors">
                            <Clock className="w-4 h-4" />
                          </button>
                          <button className="text-neutral-500 hover:text-white transition-colors">
                            <RotateCcw className="w-4 h-4" />
                          </button>
                        </div>
                      </div>
                    </div>

                    <div className="text-center text-neutral-500 text-sm">
                      Mar 10, 2025, 2:14 PM
                    </div>

                    {/* Reply Section */}
                    <div className="space-y-2">
                      <p className="text-neutral-500 text-sm">
                        [user name] replied to you
                      </p>
                      <div className="bg-neutral-700 rounded-lg p-4 space-y-3">
                        <div className="flex items-center justify-between">
                          <h4 className="font-semibold text-white">
                            [Post Title]
                          </h4>
                          <span className="text-neutral-500 text-xs">
                            1w ago
                          </span>
                        </div>

                        <div className="flex items-center gap-2">
                          <Avatar className="w-6 h-6">
                            <AvatarFallback className="bg-primary text-black text-xs">
                              P
                            </AvatarFallback>
                          </Avatar>
                          <span className="text-sm text-white">
                            Politician name
                          </span>
                          <Star className="w-4 h-4 text-primary fill-current" />
                        </div>

                        <p className="text-white text-sm leading-relaxed">
                          Life isn't a straight road, and it's not supposed to
                          be. Some turns teach you patience, some dead ends
                          build your strength. It's not always about moving...
                        </p>
                      </div>
                    </div>
                  </div>
                </div>
              ) : (
                /* Default Message Content */
                <div className="flex flex-col items-center justify-center flex-1 min-h-96">
                  <div className="w-20 h-20 bg-neutral-800 rounded-full flex items-center justify-center mb-6">
                    <MessageCircle className="w-10 h-10 text-neutral-700" />
                  </div>
                  <h3 className="text-xl font-semibold text-white mb-2">
                    Your messages
                  </h3>
                  <p className="text-neutral-500 text-center mb-6">
                    Send a message to start a chat.
                  </p>
                  <Button className="bg-white text-black hover:bg-gray-200 px-8 py-2 rounded-full font-medium">
                    Send Message
                  </Button>
                </div>
              )}
            </div>
          </div>
        </div>
        // <div className="grid grid-cols-2 gap-6">

        // </div>
      )}
    </div>
  );
}
