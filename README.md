# An example product created with vibe-coding
This product is an example create with [vibe-coding](https://en.wikipedia.org/wiki/Vibe_coding).

## High level overview of GroceryNana
Here is my software idea: A software for general users to create grocery lists and for local shops to see these grocery lists and send offers to users. Grocery lists should not be public. Users, both buyers and sellers, have to register using their phone number. Phone numbers should be verified with a one time code. Any user can create grocery list. But in order to send offers to fulfill the grocery list, a seller needs to be manually verified. I need an admin interface for this. The admin should be limited only to my email address (provided later).

## Clarification questions (from Claude.ai)
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
5. Sellers have to provide business license, full address and picture of their storefront
6. There should only be one list per Buyer at the moment and it should expire in a week. Expiry date should be visible to all parties involved
