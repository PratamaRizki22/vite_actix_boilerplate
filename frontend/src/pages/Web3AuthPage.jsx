import React, { useState } from 'react';
import { useNavigate, Link } from 'react-router-dom';
import { useAuth } from '../context/AuthContext';

const Web3AuthPage = () => {
  const [address, setAddress] = useState('');
  const [challenge, setChallenge] = useState('');
  const [step, setStep] = useState('select'); // select, challenge, verify
  const [signature, setSignature] = useState('');
  const [error, setError] = useState('');
  const [loading, setLoading] = useState(false);
  
  const navigate = useNavigate();
  const { getWeb3Challenge, verifyWeb3Signature } = useAuth();

  // Check if MetaMask is available
  const isMetaMaskAvailable = () => {
    return typeof window.ethereum !== 'undefined';
  };

  // Check and switch to testnet
  const checkAndSwitchToTestnet = async () => {
    if (!isMetaMaskAvailable()) {
      return false;
    }

    try {
      const chainId = await window.ethereum.request({
        method: 'eth_chainId',
      });

      // Sepolia testnet chain ID is 0xaa36a7 (11155111 in decimal)
      const SEPOLIA_CHAIN_ID = '0xaa36a7';

      if (chainId !== SEPOLIA_CHAIN_ID) {
        try {
          await window.ethereum.request({
            method: 'wallet_switchEthereumChain',
            params: [{ chainId: SEPOLIA_CHAIN_ID }],
          });
        } catch (switchError) {
          if (switchError.code === 4902) {
            // Chain doesn't exist, add it
            await window.ethereum.request({
              method: 'wallet_addEthereumChain',
              params: [
                {
                  chainId: SEPOLIA_CHAIN_ID,
                  chainName: 'Sepolia Testnet',
                  rpcUrls: ['https://sepolia.infura.io/v3/YOUR_INFURA_KEY'],
                  blockExplorerUrls: ['https://sepolia.etherscan.io'],
                  nativeCurrency: {
                    name: 'Sepolia ETH',
                    symbol: 'ETH',
                    decimals: 18,
                  },
                },
              ],
            });
          } else {
            throw switchError;
          }
        }
      }
      return true;
    } catch (err) {
      setError('Please switch to Sepolia Testnet in MetaMask');
      return false;
    }
  };

  // Connect MetaMask and get address
  const connectMetaMask = async () => {
    if (!isMetaMaskAvailable()) {
      setError('MetaMask is not installed. Please install it to continue.');
      return;
    }

    setLoading(true);
    setError('');

    try {
      // Check and switch to testnet first
      const isTestnet = await checkAndSwitchToTestnet();
      if (!isTestnet) {
        setLoading(false);
        return;
      }

      const accounts = await window.ethereum.request({
        method: 'eth_requestAccounts',
      });
      if (accounts && accounts.length > 0) {
        setAddress(accounts[0]);
        setStep('challenge');
      }
    } catch (err) {
      setError(err.message || 'Failed to connect MetaMask');
    } finally {
      setLoading(false);
    }
  };

  // Request challenge from backend
  const requestChallenge = async () => {
    setLoading(true);
    setError('');

    try {
      const response = await getWeb3Challenge(address);
      setChallenge(response.challenge);
      setStep('verify');
    } catch (err) {
      setError(err.response?.data?.error || 'Failed to get challenge');
    } finally {
      setLoading(false);
    }
  };

  // Sign challenge with MetaMask
  const signChallenge = async () => {
    if (!isMetaMaskAvailable()) {
      setError('MetaMask is not available');
      return;
    }

    setLoading(true);
    setError('');

    try {
      const messageHash = challenge;
      const sig = await window.ethereum.request({
        method: 'personal_sign',
        params: [messageHash, address],
      });
      setSignature(sig);
      await verifySignature(sig);
    } catch (err) {
      setError(err.message || 'Failed to sign challenge');
    } finally {
      setLoading(false);
    }
  };

  // Verify signature with backend
  const verifySignature = async (sig) => {
    setLoading(true);
    setError('');

    try {
      const response = await verifyWeb3Signature(address, challenge, sig);
      
      // Check if user has 2FA enabled
      if (response.user && response.user.two_factor_enabled) {
        // Redirect to 2FA verification page
        navigate('/2fa-verify', { state: { username: response.user.username } });
      } else {
        // No 2FA → navigate to dashboard
        navigate('/dashboard');
      }
    } catch (err) {
      setError(err.response?.data?.error || 'Signature verification failed');
    } finally {
      setLoading(false);
    }
  };

  return (
      <div className="min-h-screen bg-white flex items-center justify-center p-4">
      <div className="w-full max-w-md border border-black p-8">
        <h1 className="text-3xl font-bold text-black mb-2 text-center">Web3 Authentication</h1>
        <p className="text-center text-black text-sm mb-6">Sepolia Testnet Only</p>

        {error && (
          <div className="border border-black bg-white p-4 mb-6 text-black">
            {error}
          </div>
        )}

        {step === 'select' && (
          <div className="space-y-4">
            <p className="text-black mb-6">Connect your MetaMask wallet to authenticate on Sepolia Testnet.</p>
            {!isMetaMaskAvailable() ? (
              <div className="border border-black p-4 bg-white mb-4">
                <p className="text-black font-bold mb-2">MetaMask not installed</p>
                <p className="text-black text-sm mb-4">
                  You need MetaMask to use Web3 authentication. Download it now:
                </p>
                <a
                  href="https://metamask.io/download/"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="block text-center bg-black text-white border border-black font-bold py-2 px-4 hover:bg-white hover:text-black transition"
                >
                  Download MetaMask
                </a>
              </div>
            ) : (
              <button
                onClick={connectMetaMask}
                disabled={loading}
                className="w-full bg-white border border-black text-black font-bold py-2 px-4 hover:bg-black hover:text-white transition disabled:opacity-50"
              >
                {loading ? 'Connecting...' : 'Connect MetaMask'}
              </button>
            )}
            <p className="text-black text-xs">
              Make sure you're on Sepolia Testnet. Get test ETH at{' '}
              <a
                href="https://cloud.google.com/application/web3/faucet/ethereum/sepolia"
                target="_blank"
                rel="noopener noreferrer"
                className="font-bold border-b border-black hover:bg-black hover:text-white"
              >
                Google Cloud Faucet
              </a>
            </p>
          </div>
        )}

        {step === 'challenge' && (
          <div className="space-y-4">
            <div className="border border-black p-4 bg-white">
              <p className="text-black text-sm font-bold mb-2">Connected Address:</p>
              <p className="text-black text-xs break-all font-mono">{address}</p>
            </div>
            <div className="border border-black p-4 bg-white">
              <p className="text-black text-sm font-bold mb-2">Network:</p>
              <p className="text-black text-xs">Sepolia Testnet</p>
            </div>
            <p className="text-black text-sm">Click the button below to request a challenge.</p>
            <button
              onClick={requestChallenge}
              disabled={loading}
              className="w-full bg-white border border-black text-black font-bold py-2 px-4 hover:bg-black hover:text-white transition disabled:opacity-50"
            >
              {loading ? 'Getting challenge...' : 'Request Challenge'}
            </button>
          </div>
        )}

        {step === 'verify' && (
          <div className="space-y-4">
            <div className="border border-black p-4 bg-white">
              <p className="text-black text-sm font-bold mb-2">Challenge:</p>
              <p className="text-black text-xs break-all font-mono">{challenge}</p>
            </div>
            <p className="text-black text-sm">Sign the challenge with MetaMask to complete authentication.</p>
            <button
              onClick={signChallenge}
              disabled={loading}
              className="w-full bg-white border border-black text-black font-bold py-2 px-4 hover:bg-black hover:text-white transition disabled:opacity-50"
            >
              {loading ? 'Signing...' : 'Sign Challenge'}
            </button>
          </div>
        )}

        <div className="mt-6 text-center">
          <p className="text-black">
            <Link to="/login" className="font-bold border-b border-black hover:bg-black hover:text-white">
              Back to Login
            </Link>
          </p>
        </div>
      </div>
    </div>
  );
};

export default Web3AuthPage;
