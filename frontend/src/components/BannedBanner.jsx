import { useAuth } from '../context/AuthContext'

const BannedBanner = () => {
    const { user } = useAuth()

    if (!user || !user.is_banned) return null

    const getBanMessage = () => {
        if (user.banned_until) {
            const banDate = new Date(user.banned_until)
            const now = new Date()

            if (banDate < now) {
                return "Your ban has expired. Please refresh the page."
            }

            return `Your account is banned until ${banDate.toLocaleString()}. You can view content but cannot post, comment, or interact.`
        }

        return "Your account has been permanently banned. You can view content but cannot post, comment, or interact."
    }

    return (
        <div className="bg-red-500 border-4 border-black text-white p-4 mb-4 mt-20">
            <div className="max-w-7xl mx-auto">
                <h2 className="text-xl font-bold mb-2">⚠️ Account Banned</h2>
                <p className="font-bold">{getBanMessage()}</p>
                <p className="mt-2 text-sm">If you believe this is a mistake, please contact support.</p>
            </div>
        </div>
    )
}

export default BannedBanner
