import React from "react";
import withModulesState from "../lib/withModulesState";

/**
 * React Component which Displays a video thumbnail centered and on top
 * of the position wanted.
 *
 * Takes 2 props:
 *   - {Object} Adaptation - The adaptation that carries the thumbnail track
 *   - {number} Image time - The media time of the image to display
 *
 * @class VideoThumbnailTip
 */
class VideoThumbnail extends React.Component {
  constructor(...args) {
    super(...args);
    this.positionIsCorrected = false;
    this.state = {
      style: {},
    };
    this.isSettingTime = false;
    this._videoElement = undefined;
  }

  correctImagePosition() {
    if (this.positionIsCorrected) {
      return;
    }
    const { xPosition } = this.props;

    if (isNaN(+xPosition) || !this.element) {
      return null;
    }

    const style = {
      transform: `translate(${xPosition}px, -136px)`,
    };

    this.positionIsCorrected = true;
    this.setState({ style });
  }

  componentWillReceiveProps() {
    this.positionIsCorrected = false;
  }

  componentDidMount() {
    this.correctImagePosition();
    if (this._videoElement !== undefined) {
      this.props.player.dispatch("ATTACH_VIDEO_THUMBNAIL_LOADER", this._videoElement);
    }
  }

  componentDidUpdate() {
    this.correctImagePosition();
  }

  componentWillUnmount() {
    const { player, attachedVideoThumbnailLoader } = this.props;
    const videoThumbnailLoader = attachedVideoThumbnailLoader;
    if (videoThumbnailLoader) {
      videoThumbnailLoader.dispose();
      player.dispatch("REMOVE_VIDEO_THUMBNAIL_LOADER");
    }
    this._videoElement = undefined;
  }

  render() {
    const { style } = this.state;

    const videoThumbnailLoader = this.props.attachedVideoThumbnailLoader;
    if (videoThumbnailLoader) {
      const { time } = this.props;
      if (!this.isSettingTime) {
        this.isSettingTime = true;
        videoThumbnailLoader.setTime(Math.floor(time))
          .then(() => this.isSettingTime = false)
          .catch(() => this.isSettingTime = false);
      }
    }

    const divToDisplay = <div
      className="thumbnail-wrapper"
      style={style}
      ref={el => this.element = el}
    >
      <video ref={(videoElement) => {
        if (videoElement !== null) {
          this._videoElement = videoElement;
        }
      }}></video>
    </div>;

    return (
      divToDisplay
    );
  }
}

export default React.memo(withModulesState({
  player: {
    attachedVideoThumbnailLoader: "attachedVideoThumbnailLoader"
  },
})(VideoThumbnail));