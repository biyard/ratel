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
  Forward,
  Reply
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

  const [showForwardModal, setShowForwardModal] = useState(false)
  const [showReplyModal, setShowReplyModal] = useState(false)
  const [selectedContacts, setSelectedContacts] = useState<string[]>([])
  const [searchQuery, setSearchQuery] = useState("")
  const [conversations, setConversations] = useState<Record<string, any[]>>({})

  const handleUserClick = (userName: string) => {
    setSelectedUser(userName);
  };

  const handleCloseConversation = () => {
    setSelectedUser(null);
  };

  const handleForwardClick = () => {
    setShowForwardModal(true)
  }

  const handleReplyClick = () => {
    setShowReplyModal(true)
  }

  const handleCloseForwardModal = () => {
    setShowForwardModal(false)
    setSelectedContacts([])
    setSearchQuery("")
  }

  const handleCloseReplyModal = () => {
    setShowReplyModal(false)
    setSelectedContacts([])
    setSearchQuery("")
  }

  const handleContactSelect = (contactId: string) => {
    setSelectedContacts((prev) =>
      prev.includes(contactId) ? prev.filter((id) => id !== contactId) : [...prev, contactId],
    )
  }

  const handleSendForward = () => {
    if (selectedContacts.length > 0 && selectedUser) {
      // Create the forwarded message object
      const forwardedMessage = {
        id: Date.now(),
        type: "forwarded",
        timestamp: new Date().toLocaleString("en-US", {
          month: "short",
          day: "numeric",
          year: "numeric",
          hour: "numeric",
          minute: "2-digit",
          hour12: true,
        }),
        content: {
          title: "[Post Title]",
          author: "Politician name",
          authorTime: "1w ago",
          text: "Life isn't a straight road, and it's not supposed to be. Some turns teach you patience, some dead ends build your strength. It's not always about moving fast—it's about moving with meaning. Even when you feel lost, you're gathering pieces of yourself along the way. Every mistake, every delay, every unexpected moment is shaping a version of you that's wiser, kinder, and more real. You don't need to have it all figured out. You just need to keep showing up for yourself, one honest step at a time.",
          reply: "It's our place!",
        },
        forwardedTo: selectedContacts.map((id) => suggestedContacts.find((c) => c.id === id)?.name).filter(Boolean),
      }

      // Add the message to the current conversation
      setConversations((prev) => ({
        ...prev,
        [selectedUser]: [...(prev[selectedUser] || []), forwardedMessage],
      }))

      console.log("Forwarding to:", selectedContacts)
      handleCloseForwardModal()
    }
  }

  const handleSendReply = () => {
    if (selectedContacts.length > 0 && selectedUser) {
      // Create the reply message object
      const replyMessage = {
        id: Date.now(),
        type: "reply",
        timestamp: new Date().toLocaleString("en-US", {
          month: "short",
          day: "numeric",
          year: "numeric",
          hour: "numeric",
          minute: "2-digit",
          hour12: true,
        }),
        content: {
          text: "Thanks for sharing this!",
        },
        repliedTo: selectedContacts.map((id) => suggestedContacts.find((c) => c.id === id)?.name).filter(Boolean),
      }

      // Add the message to the current conversation
      setConversations((prev) => ({
        ...prev,
        [selectedUser]: [...(prev[selectedUser] || []), replyMessage],
      }))

      console.log("Replying to:", selectedContacts)
      handleCloseReplyModal()
    }
  }

  const suggestedContacts = [
    { id: "group1", name: "[Group name]", handle: "@heythisisyourid, @r3jdwofk @re er...", isGroup: true },
    { id: "hyejin1", name: "Hyejin Choi", handle: "@heythisisyourid", isGroup: false },
    { id: "hyejin2", name: "Hyejin Choi", handle: "@heythisisyourid", isGroup: false },
    { id: "hyejin3", name: "Hyejin Choi", handle: "@heythisisyourid", isGroup: false },
    { id: "hyejin4", name: "Hyejin Choi", handle: "@heythisisyourid", isGroup: false },
    { id: "hyejin5", name: "Hyejin Choi", handle: "@heythisisyourid", isGroup: false },
  ]


  return (
    <div className="col-span-2 space-y-4">
      {/* Tab Headers */}
      <div className="bg px-6 py-4 rounded-t-lg">
        <div className="grid grid-cols-2 gap-0">
          <div className="flex justify-center">
            <button
              onClick={() => setActiveTab('notification')}
              className={`text-xl font-semibold transition-colors relative ${activeTab === 'notification'
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
              className={`text-xl font-semibold transition-colors relative ${activeTab === 'message'
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
                  <div className="flex-1 overflow-y-auto p-4 space-y-4 bg rounded-b-lg">
                    {/* Post Content */}
                    <div className="bg-neutral-700 rounded-lg p-4 space-y-4">
                      <div className="flex items-center justify-between">
                        <h3 className="font-semibold text-white">[Post Title]</h3>
                        <span className="text-neutral-500 text-xs">1w ago</span>
                      </div>

                      <div className="flex items-center gap-2">
                        <Avatar className="w-6 h-6">
                          <AvatarFallback className="bg-primary text-black text-xs">P</AvatarFallback>
                        </Avatar>
                        <span className="text-sm text-white">Politician name</span>
                        <Star className="w-4 h-4 text-primary fill-current" />
                      </div>

                      <p className="text-neutral-900 text-sm leading-relaxed">
                        Life isn't a straight road, and it's not supposed to be. Some turns teach you patience, some
                        dead ends build your strength. It's not always about moving fast—it's about moving with
                        meaning. Even when you feel lost, you're gathering pieces of yourself along the way. Every
                        mistake, every delay, every unexpected moment is shaping a version of you that's wiser,
                        kinder, and more real. You don't need to have it all figured out. You just need to keep
                        showing up for yourself, one honest step at a time.
                      </p>

                      <div className="flex items-center gap-4">
                        <div className="flex items-center gap-2">
                          <Avatar className="w-6 h-6">
                            <AvatarFallback className="bg-primary text-black text-xs">R</AvatarFallback>
                          </Avatar>
                          <span className="text-sm text-white">It's our place!</span>
                        </div>
                        <div className="flex items-center gap-3 ml-auto">
                          <button className="flex items-center gap-1 text-neutral-500 hover:text-red-500 transition-colors">
                            <Heart className="w-4 h-4" />
                            <span className="text-xs">1</span>
                          </button>
                          <button className="text-neutral-500 hover:text-white transition-colors">
                            <Clock className="w-4 h-4" />
                          </button>
                          <button
                            className="text-neutral-500 hover:text-white transition-colors"
                            onClick={handleForwardClick}
                          >
                            <Forward className="w-4 h-4" />
                          </button>
                        </div>
                      </div>
                    </div>

                    <div className="text-center text-neutral-500 text-sm">Mar 10, 2025, 2:14 PM</div>

                    {/* Reply Section */}
                    <div className="space-y-2">
                      <p className="text-neutral-500 text-sm">[user name] replied to you</p>
                      <div className="bg-neutral-700 rounded-lg p-4 space-y-3">
                        <div className="flex items-center justify-between">
                          <h4 className="font-semibold text-white">[Post Title]</h4>
                          <span className="text-neutral-500 text-xs">1w ago</span>
                        </div>

                        <div className="flex items-center gap-2">
                          <Avatar className="w-6 h-6">
                            <AvatarFallback className="bg-primary text-black text-xs">P</AvatarFallback>
                          </Avatar>
                          <span className="text-sm text-white">Politician name</span>
                          <Star className="w-4 h-4 text-primary fill-current" />
                        </div>

                        <p className="text-neutral-900 text-sm leading-relaxed">
                          Life isn't a straight road, and it's not supposed to be. Some turns teach you patience, some
                          dead ends build your strength. It's not always about moving...
                        </p>

                        {/* Added Reply Button */}
                        <div className="flex justify-end">
                          <button
                            className="text-neutral-500 hover:text-white transition-colors"
                            onClick={handleReplyClick}
                          >
                            <Reply className="w-4 h-4" />
                          </button>
                        </div>
                      </div>
                    </div>

                    {/* Forwarded Messages */}
                    {conversations[selectedUser]?.map((message) =>
                      message.type === "forwarded" ? (
                        <div key={message.id} className="space-y-2">
                          <div className="text-center text-neutral-500 text-sm">{message.timestamp}</div>
                          <div className="text-neutral-500 text-sm">You forwarded a message</div>
                          <div className="bg-neutral-700 rounded-lg p-4 space-y-4 border-l-4 border-primary">
                            <div className="flex items-center justify-between">
                              <h3 className="font-semibold text-white">{message.content.title}</h3>
                              <span className="text-neutral-500 text-xs">{message.content.authorTime}</span>
                            </div>

                            <div className="flex items-center gap-2">
                              <Avatar className="w-6 h-6">
                                <AvatarFallback className="bg-primary text-black text-xs">P</AvatarFallback>
                              </Avatar>
                              <span className="text-sm text-white">{message.content.author}</span>
                              <Star className="w-4 h-4 text-primary fill-current" />
                            </div>

                            <p className="text-neutral-900 text-sm leading-relaxed">{message.content.text}</p>

                            <div className="flex items-center gap-4">
                              <div className="flex items-center gap-2">
                                <Avatar className="w-6 h-6">
                                  <AvatarFallback className="bg-primary text-black text-xs">R</AvatarFallback>
                                </Avatar>
                                <span className="text-sm text-white">{message.content.reply}</span>
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
                                  <Forward className="w-4 h-4" />
                                </button>
                              </div>
                            </div>
                          </div>
                        </div>
                      ) : null,
                    )}
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
      )}

      {/* Forward Modal */}
            {showForwardModal && (
              <div className="fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center z-50">
                <div className="bg rounded-lg w-full max-w-md mx-4">
                  {/* Modal Header */}
                  <div className="flex items-center justify-between p-4 border-b border-neutral-700">
                    <h2 className="text-xl font-semibold text-white">Forward</h2>
                    <button onClick={handleCloseForwardModal} className="text-neutral-500 hover:text-white transition-colors">
                      <X className="w-5 h-5" />
                    </button>
                  </div>
      
                  {/* Modal Content */}
                  <div className="p-4 space-y-4">
                    {/* To Field with Tags */}
                    <div>
                      <label className="text-white text-sm font-medium mb-2 block">To:</label>
                      <div className="min-h-[40px] bg-neutral-800 border border-neutral-700 rounded-md p-2 flex flex-wrap gap-2 items-center">
                        {/* Selected Contact Tags */}
                        {selectedContacts.map((contactId) => {
                          const contact = suggestedContacts.find((c) => c.id === contactId)
                          return contact ? (
                            <div
                              key={contactId}
                              className="bg-primary text-black px-3 py-1 rounded-full text-sm font-medium flex items-center gap-2"
                            >
                              <span>{contact.name}</span>
                              <button
                                onClick={() => handleContactSelect(contactId)}
                                className="hover:bg-black/20 rounded-full p-0.5 transition-colors"
                              >
                                <X className="w-3 h-3" />
                              </button>
                            </div>
                          ) : null
                        })}
      
                        {/* Search Input */}
                        <div className="flex-1 min-w-[100px]">
                          <input
                            type="text"
                            placeholder="Search..."
                            value={searchQuery}
                            onChange={(e) => setSearchQuery(e.target.value)}
                            className="w-full bg-transparent text-white placeholder-neutral-500 outline-none text-sm"
                          />
                        </div>
                      </div>
                    </div>
      
                    {/* Suggested Section */}
                    <div>
                      <h3 className="text-white text-sm font-medium mb-3">Suggested</h3>
                      <div className="space-y-3 max-h-64 overflow-y-auto custom-scrollbar">
                        {suggestedContacts
                          .filter(
                            (contact) =>
                              searchQuery === "" ||
                              contact.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
                              contact.handle.toLowerCase().includes(searchQuery.toLowerCase()),
                          )
                          .map((contact) => (
                            <div
                              key={contact.id}
                              className="flex items-center gap-3 p-2 hover:bg-neutral-800 rounded-lg transition-colors cursor-pointer"
                              onClick={() => handleContactSelect(contact.id)}
                            >
                              <Avatar className="w-10 h-10">
                                <AvatarFallback className="bg-neutral-700 text-white text-sm">
                                  {contact.isGroup ? "G" : contact.name.charAt(0)}
                                </AvatarFallback>
                              </Avatar>
                              <div className="flex-1 min-w-0">
                                <p className="font-medium text-white text-sm">{contact.name}</p>
                                <p className="text-neutral-500 text-xs truncate">{contact.handle}</p>
                              </div>
                              <div className="w-6 h-6 rounded-full border-2 border-neutral-500 flex items-center justify-center">
                                {selectedContacts.includes(contact.id) ? (
                                  <div className="w-4 h-4 rounded-full bg-primary flex items-center justify-center">
                                    <svg className="w-2.5 h-2.5 text-black" fill="currentColor" viewBox="0 0 20 20">
                                      <path
                                        fillRule="evenodd"
                                        d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
                                        clipRule="evenodd"
                                      />
                                    </svg>
                                  </div>
                                ) : (
                                  <div className="w-4 h-4 rounded-full border border-neutral-500"></div>
                                )}
                              </div>
                            </div>
                          ))}
                      </div>
                    </div>
                  </div>
      
                  {/* Modal Footer */}
                  <div className="p-4 border-t border-neutral-700">
                    <Button
                      onClick={handleSendForward}
                      disabled={selectedContacts.length === 0}
                      className="w-full bg-primary text-black hover:bg-primary-50 font-medium py-3 rounded-lg disabled:opacity-50 disabled:cursor-not-allowed"
                    >
                      Send
                    </Button>
                  </div>
                </div>
              </div>
            )}
    </div>
  );
}
