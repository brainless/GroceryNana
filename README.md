# An example repository for vibe-coding
This repository exists just to share how one may ask Claude.ai to discuss a software idea and then create tickets that I would then use to create the actual software using Claude Code. The original prompt and my clarifications are below. All software details were created by Claude.ai. I have copied the tickets it gave me into Issues (see Issues tab)[https://github.com/brainless/GroceryNana/issues].

## Original prompt for Claude.ai
Help me generate tickets to use in Claude Code to build my software idea. I want to create a full-stack application with automated tests. Please mention using technologies that allow building software that is as safe as possible. I will use GitHub, DigitalOcean, Cloudflare. Please create tickets to use these infrastructure, to document the usage. I am not technical so tickets to document the entire system is very important. The documentation should be easy to browse and search.

Here is my software idea: A software for users to create grocery lists and for local shops to see these grocery lists and send offers to users. Grocery lists should not be public. Users (buyers or sellers) have to register using their phone number. Phone numbers need to be verified with a one time code. Any user can create grocery list. But in order to send offers to fulfill the grocery list, a seller needs to be manually verified. I need an admin interface for this. The admin should be limited only to specific users.

Please ask me further questions to clarify the details any time. Please create tickets that I can copy and paste into GitHub issues. I will then ask Claude Code to build this software and get it hosted for me.

## Questions asked by Claude.ai
I'll help you create comprehensive tickets for your grocery list marketplace application. Let me ask a few clarifying questions first to ensure the tickets cover all your needs:

1. User experience flow: When sellers send offers, how should buyers receive and respond to them? (email notifications, in-app notifications, SMS?)
2. Offer system: Should buyers be able to accept/reject offers, negotiate prices, or is it more of a "take it or leave it" system?
3. Geographic scope: Do you want to limit sellers to specific areas/zip codes near buyers, or is this location-agnostic?
4. Payment handling: Do you want integrated payments, or should buyers/sellers handle payment separately?
5. Admin verification: What information should sellers provide for manual verification? (business license, ID, etc.)
6. Data retention: How long should grocery lists and offers be kept in the system?

## Clarification answers
Let me answers your questions:
1. Buyers and sellers can communicate only using in-app notifications and direct messing. Each message should refer to the grocery list. User should see messages per list as well as per buyer/seller
2. Yes price negotiations can happen over messages per grocery list. Seller sets a new price as a data point. Buyer can accept or ignore. If accepted, then the grocery list is marked fulfilled for all Sellers
3. Location is very important. We should ask Buyers for their range to go and pick-up
4. Let us postpone payment integration
5. Sellers have to provide business license and ID as well as picture of their storefront
6. There should only be one list per Buyer at the moment and it should expire in a week. Expiry date should be visible to all parties involved

## 🎯 Project Overview
A secure full-stack application where users create grocery lists and local shops can send offers to fulfill them. Focus on safety, security, and comprehensive documentation.

## 🚀 Implementation Priority

1. **Phase 1 - Foundation**: Tickets 1-3 (Foundation, Auth, User Management)
2. **Phase 2 - Core Features**: Tickets 4-7 (Lists, Messaging, Location, Documents)
3. **Phase 3 - Polish & Documentation**: Tickets 8-9 (Documentation, Testing)
4. **Phase 4 - Production**: Tickets 10-12 (Security, Deployment, Performance)

## 🔧 Additional Notes for Implementation

### Key Technical Considerations:
- **Location Services**: Implement efficient geographic queries with proper indexing
- **Real-time Messaging**: Consider WebSocket connections for instant messaging
- **Document Storage**: Secure file handling with proper validation and virus scanning
- **One List Per Buyer**: Implement proper constraints to enforce this business rule
