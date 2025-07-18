import React from 'react'
import { Button } from '@/components/ui/button'

const HomePage: React.FC = () => {
  return (
    <div className="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 flex items-center justify-center">
      <div className="text-center">
        <h1 className="text-6xl font-bold text-gray-900 mb-4">
          Hello World
        </h1>
        <p className="text-xl text-gray-600 mb-8">
          Welcome to GroceryNana - Your Local Grocery Marketplace
        </p>
        <div className="space-y-4">
          <p className="text-gray-700">
            Frontend successfully set up with:
          </p>
          <ul className="text-sm text-gray-600 space-y-2 mb-8">
            <li>✅ React 18 + TypeScript</li>
            <li>✅ Vite for fast development</li>
            <li>✅ Tailwind CSS for styling</li>
            <li>✅ React Router for navigation</li>
            <li>✅ Axios for API communication</li>
            <li>✅ Zustand for state management</li>
            <li>✅ React Hook Form + Zod for forms</li>
            <li>✅ shadcn/ui components</li>
          </ul>
          <div className="space-x-4">
            <Button variant="default">Get Started</Button>
            <Button variant="outline">Learn More</Button>
          </div>
        </div>
      </div>
    </div>
  )
}

export default HomePage