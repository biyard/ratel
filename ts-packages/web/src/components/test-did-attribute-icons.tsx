// Temporary test file to verify DID attribute icons work
import { Age, Gender } from './icons';

export function TestDidAttributeIcons() {
  return (
    <div className="p-4">
      <h2 className="text-xl font-bold mb-4">DID Attribute Icons Test</h2>
      <div className="space-y-8">
        {/* Age Icon */}
        <div>
          <h3 className="text-lg font-semibold mb-4">Age Icon</h3>
          <div className="flex gap-4 items-center">
            <div className="flex flex-col items-center">
              <Age className="w-6 h-6" />
              <span className="text-xs mt-2">Default (24px)</span>
            </div>
            <div className="flex flex-col items-center">
              <Age className="w-8 h-8" />
              <span className="text-xs mt-2">Large (32px)</span>
            </div>
            <div className="flex flex-col items-center">
              <Age className="w-12 h-12" />
              <span className="text-xs mt-2">Extra Large (48px)</span>
            </div>
            <div className="flex flex-col items-center">
              <Age className="w-6 h-6 text-blue-500" />
              <span className="text-xs mt-2">Blue</span>
            </div>
            <div className="flex flex-col items-center">
              <Age className="w-6 h-6 text-green-500" />
              <span className="text-xs mt-2">Green</span>
            </div>
          </div>
        </div>

        {/* Gender Icon */}
        <div>
          <h3 className="text-lg font-semibold mb-4">Gender Icon</h3>
          <div className="flex gap-4 items-center">
            <div className="flex flex-col items-center">
              <Gender className="w-6 h-6" />
              <span className="text-xs mt-2">Default (24px)</span>
            </div>
            <div className="flex flex-col items-center">
              <Gender className="w-8 h-8" />
              <span className="text-xs mt-2">Large (32px)</span>
            </div>
            <div className="flex flex-col items-center">
              <Gender className="w-12 h-12" />
              <span className="text-xs mt-2">Extra Large (48px)</span>
            </div>
            <div className="flex flex-col items-center">
              <Gender className="w-6 h-6 text-purple-500" />
              <span className="text-xs mt-2">Purple</span>
            </div>
            <div className="flex flex-col items-center">
              <Gender className="w-6 h-6 text-pink-500" />
              <span className="text-xs mt-2">Pink</span>
            </div>
          </div>
        </div>

        {/* Combined Usage */}
        <div>
          <h3 className="text-lg font-semibold mb-4">Usage Examples</h3>
          <div className="space-y-4">
            <div className="flex items-center gap-3 p-3 bg-gray-100 rounded">
              <Age className="w-8 h-8" />
              <div>
                <p className="font-medium">Age Verification</p>
                <p className="text-sm text-gray-600">20 - 29 years old</p>
              </div>
            </div>
            <div className="flex items-center gap-3 p-3 bg-gray-100 rounded">
              <Gender className="w-8 h-8" />
              <div>
                <p className="font-medium">Gender Verification</p>
                <p className="text-sm text-gray-600">Registration required</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
